//! Provides functions to parse input IP addresses, CIDRs or files.
use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::str::FromStr;

use cidr_utils::cidr::IpCidr;
use hickory_resolver::{
    config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts},
    Resolver,
};
use log::debug;

use crate::input::Opts;
use crate::warning;

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
pub fn parse_addresses(input: &Opts) -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = Vec::new();
    let mut unresolved_addresses: Vec<&str> = Vec::new();
    let backup_resolver = get_resolver(&input.resolver);

    for address in &input.addresses {
        let parsed_ips = parse_address(address, &backup_resolver);
        if !parsed_ips.is_empty() {
            ips.extend(parsed_ips);
        } else {
            unresolved_addresses.push(address);
        }
    }

    // If we got to this point this can only be a file path or the wrong input.
    for file_path in unresolved_addresses {
        let file_path = Path::new(file_path);

        if !file_path.is_file() {
            warning!(
                format!("Host {file_path:?} could not be resolved."),
                input.greppable,
                input.accessible
            );

            continue;
        }

        if let Ok(x) = read_ips_from_file(file_path, &backup_resolver) {
            ips.extend(x);
        } else {
            warning!(
                format!("Host {file_path:?} could not be resolved."),
                input.greppable,
                input.accessible
            );
        }
    }

    ips
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
/// # use hickory_resolver::Resolver;
/// let ips = parse_address("127.0.0.1", &Resolver::default().unwrap());
/// ```
pub fn parse_address(address: &str, resolver: &Resolver) -> Vec<IpAddr> {
    IpCidr::from_str(address)
        .map(|cidr| cidr.iter().map(|c| c.address()).collect())
        .ok()
        .or_else(|| {
            format!("{}:{}", &address, 80)
                .to_socket_addrs()
                .ok()
                .map(|mut iter| vec![iter.next().unwrap().ip()])
        })
        .unwrap_or_else(|| resolve_ips_from_host(address, resolver))
}

/// Uses DNS to get the IPS associated with host
fn resolve_ips_from_host(source: &str, backup_resolver: &Resolver) -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = Vec::new();

    if let Ok(addrs) = source.to_socket_addrs() {
        for ip in addrs {
            ips.push(ip.ip());
        }
    } else if let Ok(addrs) = backup_resolver.lookup_ip(source) {
        ips.extend(addrs.iter());
    }

    ips
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
fn get_resolver(resolver: &Option<String>) -> Resolver {
    match resolver {
        Some(r) => {
            let mut config = ResolverConfig::new();
            let resolver_ips = match read_resolver_from_file(r) {
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
            Resolver::new(config, ResolverOpts::default()).unwrap()
        }
        None => match Resolver::from_system_conf() {
            Ok(resolver) => resolver,
            Err(_) => {
                Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap()
            }
        },
    }
}

/// Parses and input file of IPs for use in DNS resolution.
fn read_resolver_from_file(path: &str) -> Result<Vec<IpAddr>, std::io::Error> {
    let ips = fs::read_to_string(path)?
        .lines()
        .filter_map(|line| IpAddr::from_str(line.trim()).ok())
        .collect();

    Ok(ips)
}

#[cfg(not(tarpaulin_include))]
/// Parses an input file of IPs and uses those
fn read_ips_from_file(
    ips: &std::path::Path,
    backup_resolver: &Resolver,
) -> Result<Vec<IpAddr>, std::io::Error> {
    let file = File::open(ips)?;
    let reader = BufReader::new(file);

    let mut ips: Vec<IpAddr> = Vec::new();

    for address_line in reader.lines() {
        if let Ok(address) = address_line {
            ips.extend(parse_address(&address, backup_resolver));
        } else {
            debug!("Line in file is not valid");
        }
    }

    Ok(ips)
}

#[cfg(test)]
mod tests {
    use super::{get_resolver, parse_addresses, Opts};
    use std::net::Ipv4Addr;

    #[test]
    fn parse_correct_addresses() {
        let mut opts = Opts::default();
        opts.addresses = vec!["127.0.0.1".to_owned(), "192.168.0.0/30".to_owned()];
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
        let mut opts = Opts::default();
        opts.addresses = vec!["google.com".to_owned()];
        let ips = parse_addresses(&opts);

        assert_eq!(ips.len(), 1);
    }

    #[test]
    fn parse_correct_and_incorrect_addresses() {
        let mut opts = Opts::default();
        opts.addresses = vec!["127.0.0.1".to_owned(), "im_wrong".to_owned()];
        let ips = parse_addresses(&opts);

        assert_eq!(ips, [Ipv4Addr::new(127, 0, 0, 1),]);
    }

    #[test]
    fn parse_incorrect_addresses() {
        let mut opts = Opts::default();
        opts.addresses = vec!["im_wrong".to_owned(), "300.10.1.1".to_owned()];
        let ips = parse_addresses(&opts);

        assert!(ips.is_empty());
    }
    #[test]
    fn parse_hosts_file_and_incorrect_hosts() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let mut opts = Opts::default();
        opts.addresses = vec!["fixtures/hosts.txt".to_owned()];
        let ips = parse_addresses(&opts);
        assert_eq!(ips.len(), 3);
    }

    #[test]
    fn parse_empty_hosts_file() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let mut opts = Opts::default();
        opts.addresses = vec!["fixtures/empty_hosts.txt".to_owned()];
        let ips = parse_addresses(&opts);
        assert_eq!(ips.len(), 0);
    }

    #[test]
    fn parse_naughty_host_file() {
        // Host file contains IP, Hosts, incorrect IPs, incorrect hosts
        let mut opts = Opts::default();
        opts.addresses = vec!["fixtures/naughty_string.txt".to_owned()];
        let ips = parse_addresses(&opts);
        assert_eq!(ips.len(), 0);
    }

    #[test]
    fn resolver_default_cloudflare() {
        let opts = Opts::default();

        let resolver = get_resolver(&opts.resolver);
        let lookup = resolver.lookup_ip("www.example.com.").unwrap();

        assert!(opts.resolver.is_none());
        assert!(lookup.iter().next().is_some());
    }

    #[test]
    fn resolver_args_google_dns() {
        let mut opts = Opts::default();
        // https://developers.google.com/speed/public-dns
        opts.resolver = Some("8.8.8.8,8.8.4.4".to_owned());

        let resolver = get_resolver(&opts.resolver);
        let lookup = resolver.lookup_ip("www.example.com.").unwrap();

        assert!(lookup.iter().next().is_some());
    }
}
