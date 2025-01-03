//! Provides functions to parse input IP addresses, CIDRs or files.

use std::cell::RefCell;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::pin::Pin;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use cidr_utils::cidr::IpCidr;
use futures::{FutureExt, StreamExt};
use hickory_resolver::{config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts}, AsyncResolver, TokioAsyncResolver};
use itertools::Either;
use log::debug;
use tokio::fs;
use tokio::io::AsyncBufReadExt;
use crate::input::Opts;
use crate::warning;

async fn parse_addresses_inner(input: &Opts) -> Vec<IpAddr> {
    let ips = &RefCell::new(Vec::new());
    let unresolved_addresses = &RefCell::new(Vec::new());
    let backup_resolver = &get_resolver(&input.resolver).await;

    futures::stream::iter(&input.addresses).for_each_concurrent(Some(4), |address| async move {
        let mut found_ip = false;
        let parsed_ips = parse_address(address, backup_resolver)
            .await.inspect(|_| found_ip = true);
        
        ips.borrow_mut().extend(parsed_ips);

        if !found_ip {
            unresolved_addresses.borrow_mut().push(address);
        }
    }).await;


    // If we got to this point this can only be a file path or the wrong input.
    futures::stream::iter(unresolved_addresses.take()).for_each_concurrent(Some(4), |file_path| async move {
        let file_path = Path::new(file_path);

        if !tokio::fs::metadata(file_path).await.map_or(false, |m| m.is_file()) {
            warning!(
                format!("Host {file_path:?} could not be resolved."),
                input.greppable,
                input.accessible
            );

            return;
        }

        if let Ok(x) = read_ips_from_file(file_path, backup_resolver).await {
            ips.borrow_mut().extend(x);
        } else {
            warning!(
                format!("Host {file_path:?} could not be resolved."),
                input.greppable,
                input.accessible
            );
        }
    }).await;

    let mut ips = ips.take();
    
    ips.sort_unstable();
    ips.dedup();
    ips
}

/// Parses the string(s) into IP addresses.
///
/// Goes through all possible IP inputs (files or via argparsing).
///
/// ```rust
/// # use rustscan::input::Opts;
/// # use rustscan::address::parse_addresses;
/// let mut opts = Opts::default();
/// opts.addresses = vec!["192.168.0.0/30".to_owned()];
///
/// let ips = parse_addresses(&opts);
/// ```
///
/// Finally, any duplicates are removed to avoid excessive scans.
pub fn parse_addresses(input: &Opts) -> Vec<IpAddr> {
    thread::scope(|s| {
        s.spawn(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(parse_addresses_inner(input))
        })
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    })
}

/// Given a string, parse it as a host, IP address, or CIDR.
///
/// This allows us to pass files as hosts or cidr or IPs easily
/// Call this every time you have a possible IP-or-host.
///
/// If the address is a domain, we can self-resolve the domain locally
/// or resolve it by dns resolver list.
///
/// ```rust
/// # use rustscan::address::parse_address;
/// # use hickory_resolver::{Resolver, TokioAsyncResolver};
/// # #[tokio::main]
/// # async fn main() {
/// let ips = parse_address("127.0.0.1", &TokioAsyncResolver::tokio_from_system_conf().unwrap()).await.collect::<Vec<_>>();
/// println!("{ips:?}")
/// # }
/// ```
pub async fn parse_address<'r>(address: &str, resolver: &'r TokioAsyncResolver) -> impl Iterator<Item=IpAddr>  + use<'r> {
    if let Ok(addr) = address.parse::<IpAddr>() {
        return Either::Right(std::iter::once(addr));
    }

    let res = IpCidr::from_str(address)
        .map(|cidr| cidr.iter().map(|c| c.address()))
        .ok();

    Either::Left(match res {
        Some(res) => Either::Right(res),
        None => Either::Left(resolve_ips_from_host(address, resolver).await),
    })
}

/// Uses DNS to get the IPS associated with host
async fn resolve_ips_from_host<'r>(source: &str, resolver: &'r TokioAsyncResolver) -> impl Iterator<Item=IpAddr> + use<'r> {
    let resolver_lookup = resolver.lookup_ip(source).fuse();
    let std_lookup = tokio::net::lookup_host((source.to_owned(), 80)).fuse();

    let success_iter = |data| Either::Right(data);
    let fail_iter = || Either::Left(std::iter::empty());
    
    
    let timeout = tokio::time::sleep(Duration::from_secs(10));
    
    tokio::select! {
        Ok(res) = std_lookup => success_iter(Either::Right(res.map(|s| s.ip()))),
        Ok(res) = resolver_lookup => success_iter(Either::Left(res.into_iter())),
        () = timeout => fail_iter(),
        else => fail_iter(),
    }
}

/// Derive a DNS resolver.
///
/// 1. if the `resolver` parameter has been set:
///     1. assume the parameter is a path and attempt to read IPs.
///     2. parse the input as a comma-separated list of IPs.
/// 2. if `resolver` is not set:
///    1. attempt to derive a resolver from the system config. (e.g.
///       `/etc/resolv.conf` on *nix).
///    2. finally, build a CloudFlare-based resolver (default
///       behaviour).
async fn get_resolver(resolver: &Option<String>) -> TokioAsyncResolver {
    match resolver {
        Some(r) => {
            let mut config = ResolverConfig::new();
            let resolver_ips = match read_resolver_from_file(r).await {
                Ok(ips) => ips,
                Err(_) => r
                    .split(',')
                    .filter_map(|r| IpAddr::from_str(r).ok())
                    .collect::<Vec<_>>(),
            };
            for ip in resolver_ips {
                config.add_name_server(NameServerConfig::new(
                    SocketAddr::new(ip, 53),
                    Protocol::Udp,
                ));
            }
            AsyncResolver::tokio(config, ResolverOpts::default())
        }
        None => AsyncResolver::tokio_from_system_conf().unwrap_or_else(|_| {
            AsyncResolver::tokio(ResolverConfig::cloudflare_tls(), ResolverOpts::default())
        }),
    }
}

/// Parses and input file of IPs for use in DNS resolution.
async fn read_resolver_from_file(path: &str) -> Result<Vec<IpAddr>, std::io::Error> {
    let ips = fs::read_to_string(path).await?
        .lines()
        .filter_map(|line| IpAddr::from_str(line.trim()).ok())
        .collect();

    Ok(ips)
}

#[cfg(not(tarpaulin_include))]
/// Parses an input file of IPs and uses those
async fn read_ips_from_file(
    ips: &Path,
    backup_resolver: &TokioAsyncResolver,
) -> Result<Vec<IpAddr>, std::io::Error> {
    let mut lines = tokio::io::BufReader::new(fs::File::open(ips).await?)
        .lines();

    let ips = &RefCell::new(Vec::new());
    
    let stream = futures::stream::poll_fn(move |cx| {
        Pin::new(&mut lines).poll_next_line(cx).map(|res| res.transpose())
    });
    
    stream.for_each_concurrent(Some(4), move |address_line| async move {
        if let Ok(address) = address_line {
            let addrs = parse_address(&address, backup_resolver).await;
            ips.borrow_mut().extend(addrs);
        } else {
            debug!("Line in file is not valid");
        }
    }).await;

    Ok(ips.take())
}

#[cfg(test)]
mod tests {
    use super::{get_resolver, parse_addresses, Opts};
    use std::net::Ipv4Addr;

    #[test]
    fn parse_correct_addresses() {
        let opts = Opts {
            addresses: vec!["127.0.0.1".to_owned(), "192.168.0.0/30".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(
            ips,
            [
                Ipv4Addr::new(127, 0, 0, 1),
                Ipv4Addr::new(192, 168, 0, 0),
                Ipv4Addr::new(192, 168, 0, 1),
                Ipv4Addr::new(192, 168, 0, 2),
                Ipv4Addr::new(192, 168, 0, 3)
            ]
        );
    }

    #[test]
    fn parse_correct_host_addresses() {
        let opts = Opts {
            addresses: vec!["google.com".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 1);
    }

    #[test]
    fn parse_correct_and_incorrect_addresses() {
        let opts = Opts {
            addresses: vec!["127.0.0.1".to_owned(), "im_wrong".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips, [Ipv4Addr::new(127, 0, 0, 1),]);
    }

    #[test]
    fn parse_incorrect_addresses() {
        let opts = Opts {
            addresses: vec!["im_wrong".to_owned(), "300.10.1.1".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert!(ips.is_empty());
    }
    #[test]
    fn parse_hosts_file_and_incorrect_hosts() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let opts = Opts {
            addresses: vec!["fixtures/hosts.txt".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 3);
    }

    #[test]
    fn parse_empty_hosts_file() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let opts = Opts {
            addresses: vec!["fixtures/empty_hosts.txt".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 0);
    }

    #[test]
    fn parse_naughty_host_file() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let opts = Opts {
            addresses: vec!["fixtures/naughty_string.txt".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 0);
    }

    #[test]
    fn parse_duplicate_cidrs() {
        let opts = Opts {
            addresses: vec!["79.98.104.0/21".to_owned(), "79.98.104.0/24".to_owned()],
            ..Default::default()
        };

        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 2_048);
    }

    #[tokio::test]
    async fn resolver_default_cloudflare() {
        let opts = Opts::default();

        let resolver = get_resolver(&opts.resolver).await;
        let lookup = resolver.lookup_ip("www.example.com.").await.unwrap();

        assert!(opts.resolver.is_none());
        assert!(lookup.iter().next().is_some());
    }

    #[tokio::test]
    async fn resolver_args_google_dns() {
        // https://developers.google.com/speed/public-dns
        let opts = Opts {
            resolver: Some("8.8.8.8,8.8.4.4".to_owned()),
            ..Default::default()
        };

        let resolver = get_resolver(&opts.resolver).await;
        let lookup = resolver.lookup_ip("www.example.com.").await.unwrap();

        assert!(lookup.iter().next().is_some());
    }
}
