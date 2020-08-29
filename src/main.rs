extern crate shell_words;

mod tui;

pub mod common;
use common::{PortRange, ScanOrder};

mod scanner;
use scanner::Scanner;

mod port_strategy;
use port_strategy::PortStrategy;

use colorful::Color;
use colorful::Colorful;
use futures::executor::block_on;
use rlimit::Resource;
use rlimit::{getrlimit, setrlimit};
use std::collections::HashMap;
use std::process::Command;
use std::str::FromStr;
use std::{net::IpAddr, net::ToSocketAddrs, time::Duration};
use structopt::StructOpt;

extern crate colorful;
extern crate dirs;

const LOWEST_PORT_NUMBER: u16 = 1;
const TOP_PORT_NUMBER: u16 = 65535;
// Average value for Ubuntu
const DEFAULT_FILE_DESCRIPTORS_LIMIT: rlimit::rlim = 8000;
// Safest batch size based on experimentation
const AVERAGE_BATCH_SIZE: rlimit::rlim = 3000;

#[macro_use]
extern crate log;

fn parse_range(input: &str) -> Result<PortRange, String> {
    let range = input
        .split("-")
        .map(|x| x.parse::<u16>())
        .collect::<Result<Vec<u16>, std::num::ParseIntError>>();

    if range.is_err() {
        return Err(String::from(
            "the range format must be 'start-end'. Example: 1-1000.",
        ));
    }

    match range.unwrap().as_slice() {
        [start, end] => Ok(PortRange {
            start: *start,
            end: *end,
        }),
        _ => Err(String::from(
            "the range format must be 'start-end'. Example: 1-1000.",
        )),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rustscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// Fast Port Scanner built in Rust.
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
/// - Discord https://discord.gg/GFrQsGy
/// - GitHub https://github.com/RustScan/RustScan
struct Opts {
    /// A list of comma separated IP addresses or hosts to be scanned.
    #[structopt(use_delimiter = true, required = true)]
    ips_or_hosts: Vec<String>,

    /// A list of comma separed ports to be scanned. Example: 80,443,8080.
    #[structopt(short, long, use_delimiter = true)]
    ports: Option<Vec<u16>>,

    /// A range of ports with format start-end. Example: 1-1000.
    #[structopt(short, long, conflicts_with = "ports", parse(try_from_str = parse_range))]
    range: Option<PortRange>,

    ///Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    quiet: bool,

    //Accessible mode. Turns off features which negatively affect screen readers.
    #[structopt(short, long)]
    accessible: bool,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    timeout: u32,

    /// Automatically ups the ULIMIT with the value you provided.
    #[structopt(short, long)]
    ulimit: Option<rlimit::rlim>,

    /// The order of scanning to be performed. The "serial" option will
    /// scan ports in ascending order while the "random" option will scan
    /// ports randomly.
    #[structopt(long, possible_values = &ScanOrder::variants(), case_insensitive = true, default_value = "serial")]
    scan_order: ScanOrder,

    /// The Nmap arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -t 1500 127.0.0.1 -- -A -sC'.
    /// This command adds -Pn -vvv -p $PORTS automatically to nmap.
    /// For things like --script '(safe and vuln)' enclose it in quotations marks \"'(safe and vuln)'\"")
    #[structopt(last = true)]
    command: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
/// Faster Nmap scanning with Rust
/// If you're looking for the actual scanning, check out the module Scanner
fn main() {
    env_logger::init();

    info!("Starting up");
    let mut opts = Opts::from_args();

    if opts.ports.is_none() && opts.range.is_none() {
        opts.range = Some(PortRange {
            start: LOWEST_PORT_NUMBER,
            end: TOP_PORT_NUMBER,
        });
    }

    info!("Mains() `opts` arguments are {:?}", opts);

    if !opts.quiet && !opts.accessible {
        print_opening();
    }

    let ips: Vec<IpAddr> = parse_ips(&opts);

    if ips.is_empty() {
        warning!("No IPs could be resolved, aborting scan.", false);
        std::process::exit(1);
    }

    let ulimit: rlimit::rlim = adjust_ulimit_size(&opts);
    let batch_size: u16 = infer_batch_size(&opts, ulimit);

    let scanner = Scanner::new(
        &ips,
        batch_size,
        Duration::from_millis(opts.timeout.into()),
        opts.quiet,
        PortStrategy::pick(opts.range, opts.ports, opts.scan_order),
    );

    let scan_result = block_on(scanner.run());
    let mut ports_per_ip = HashMap::new();

    for socket in scan_result {
        ports_per_ip
            .entry(socket.ip())
            .or_insert_with(Vec::new)
            .push(socket.port());
    }

    for ip in ips {
        if ports_per_ip.contains_key(&ip) {
            continue;
        }

        // If we got here it means the IP was not found within the HashMap, this
        // means the scan couldn't find any open ports for it.

        let x = format!("{} Looks like I didn't find any open ports for {:?}. This is usually caused by a high batch size.
        \n*I used {} batch size, consider lowering to {} with {} or a comfortable number for your system.
        \n Alternatively, increase the timeout if your ping is high. Rustscan -t 2000 for 2000 milliseconds (2s) timeout.\n",
        "ERROR",
        ip,
        opts.batch_size,
        (opts.batch_size / 2).to_string(),
        "'rustscan -b <batch_size> <ip address>'");
        warning!(x, opts.quiet);
    }

    for (ip, ports) in ports_per_ip.iter_mut() {
        let nmap_str_ports: Vec<String> = ports.into_iter().map(|port| port.to_string()).collect();

        detail!("Starting Nmap", opts.quiet);

        // nmap port style is 80,443. Comma separated with no spaces.
        let ports_str = nmap_str_ports.join(",");

        // if quiet mode is on nmap should not be spawned
        if opts.quiet {
            println!("{}", ports_str);
            continue;
        }

        let addr = ip.to_string();
        let user_nmap_args =
            shell_words::split(&opts.command.join(" ")).expect("failed to parse nmap arguments");
        let nmap_args = build_nmap_arguments(&addr, &ports_str, &user_nmap_args, ip.is_ipv6());

        output!(format!(
            "The Nmap command to be run is nmap {}\n",
            &nmap_args.join(" ")
        ));

        // Runs the nmap command and spawns it as a process.
        let mut child = Command::new("nmap")
            .args(&nmap_args)
            .spawn()
            .expect("failed to execute nmap process");

        child.wait().expect("failed to wait on nmap process");
    }
}

/// Prints the opening title of RustScan
fn print_opening() {
    info!("Printing opening");
    let s = r#".----. .-. .-. .----..---.  .----. .---.   .--.  .-. .-.
| {}  }| { } |{ {__ {_   _}{ {__  /  ___} / {} \ |  `| |
| .-. \| {_} |.-._} } | |  .-._} }\     }/  /\  \| |\  |
`-' `-'`-----'`----'  `-'  `----'  `---' `-'  `-'`-' `-'
Faster Nmap scanning with Rust."#;
    println!("{}", s.gradient(Color::Green).bold());
    let info = r#"________________________________________
: https://discord.gg/GFrQsGy           :
: https://github.com/RustScan/RustScan :
 --------------------------------------"#;
    println!("{}", info.gradient(Color::Yellow).bold());
    funny_opening!();

    let config_path = match dirs::config_dir() {
        Some(mut path) => {
            path.push("rustscan");
            path.push("config.toml");
            path
        }
        None => panic!("Couldn't find config dir."),
    };

    detail!(format!(
        "{} {:?}",
        "The config file is expected to be at", config_path
    ));
}
#[cfg(not(tarpaulin_include))]
fn build_nmap_arguments<'a>(
    addr: &'a str,
    ports: &'a str,
    user_args: &'a Vec<String>,
    is_ipv6: bool,
) -> Vec<&'a str> {
    let mut arguments: Vec<&str> = user_args.iter().map(AsRef::as_ref).collect();
    arguments.push("-vvv");

    if is_ipv6 {
        arguments.push("-6");
    }

    arguments.push("-p");
    arguments.push(ports);
    arguments.push(addr);

    arguments
}

fn parse_ips(opts: &Opts) -> Vec<IpAddr> {
    let mut ips: Vec<IpAddr> = Vec::new();

    for ip_or_host in &opts.ips_or_hosts {
        match IpAddr::from_str(ip_or_host) {
            Ok(ip) => ips.push(ip),
            _ => match format!("{}:{}", &ip_or_host, 80).to_socket_addrs() {
                Ok(mut iter) => ips.push(iter.nth(0).unwrap().ip()),
                _ => {
                    let failed_to_resolve = format!("Host {:?} could not be resolved.", ip_or_host);
                    warning!(failed_to_resolve, opts.quiet);
                }
            },
        }
    }

    ips
}

fn adjust_ulimit_size(opts: &Opts) -> rlimit::rlim {
    if opts.ulimit.is_some() {
        let limit: rlimit::rlim = opts.ulimit.unwrap();

        match setrlimit(Resource::NOFILE, limit, limit) {
            Ok(_) => {
                detail!(
                    format!("Automatically increasing ulimit value to {}.", limit),
                    opts.quiet
                );
            }
            Err(_) => println!("{}", "ERROR. Failed to set ulimit value."),
        }
    }

    let (rlim, _) = getrlimit(Resource::NOFILE).unwrap();

    rlim
}

fn infer_batch_size(opts: &Opts, ulimit: rlimit::rlim) -> u16 {
    let mut batch_size: rlimit::rlim = opts.batch_size.into();

    // Adjust the batch size when the ulimit value is lower than the desired batch size
    if ulimit < batch_size {
        warning!("File limit is lower than default batch size. Consider upping with --ulimit. May cause harm to sensitive servers",
            opts.quiet
        );

        // When the OS supports high file limits like 8000, but the user
        // selected a batch size higher than this we should reduce it to
        // a lower number.
        if ulimit < AVERAGE_BATCH_SIZE {
            // ulimit is smaller than aveage batch size
            // user must have very small ulimit
            // decrease batch size to half of ulimit
            warning!("Your file limit is very small, which negatively impacts RustScan's speed. Use the Docker image, or up the Ulimit with '--ulimit 5000'. ");
            info!("Halving batch_size because ulimit is smaller than average batch size");
            batch_size = ulimit / 2
        } else if ulimit > DEFAULT_FILE_DESCRIPTORS_LIMIT {
            info!("Batch size is now average batch size");
            batch_size = AVERAGE_BATCH_SIZE
        } else {
            batch_size = ulimit - 100
        }
    }
    // When the ulimit is higher than the batch size let the user know that the
    // batch size can be increased unless they specified the ulimit themselves.
    else if ulimit + 2 > batch_size && (opts.ulimit.is_none()) {
        detail!(format!(
                "File limit higher than batch size. Can increase speed by increasing batch size '-b {}'.",
                ulimit - 100
            ), opts.quiet);
    }

    batch_size as u16
}

#[cfg(test)]
mod tests {
    use crate::{adjust_ulimit_size, infer_batch_size, parse_ips, print_opening, Opts, ScanOrder};

    #[test]
    fn batch_size_lowered() {
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let batch_size = infer_batch_size(&opts, 120);

        assert!(batch_size < 50_000);
    }

    #[test]
    fn batch_size_lowered_average_size() {
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let batch_size = infer_batch_size(&opts, 9_000);

        assert!(batch_size == 3_000);
    }
    #[test]
    fn batch_size_equals_ulimit_lowered() {
        // because ulimit and batch size are same size, batch size is lowered
        // to ULIMIT - 100
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let batch_size = infer_batch_size(&opts, 5_000);

        assert!(batch_size == 4_900);
    }
    #[test]
    fn batch_size_adjusted_2000() {
        // ulimit == batch_size
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let batch_size = adjust_ulimit_size(&opts);

        assert!(batch_size == 2_000);
    }
    #[test]
    fn test_print_opening_no_panic() {
        // print opening should not panic
        print_opening();
        assert!(1 == 1);
    }
    #[test]
    fn test_high_ulimit_no_quiet_mode() {
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned()],
            ports: None,
            range: None,
            quiet: false,
            batch_size: 10,
            timeout: 1_000,
            ulimit: None,
            command: Vec::new(),
            accessible: true,
            scan_order: ScanOrder::Serial,
        };

        infer_batch_size(&opts, 1_000_000);

        assert!(1 == 1);
    }

    #[test]
    fn parse_correct_ips_or_hosts() {
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned(), "google.com".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 10,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let ips = parse_ips(&opts);

        assert_eq!(2, ips.len());
    }

    #[test]
    fn parse_correct_and_incorrect_ips_or_hosts() {
        let opts = Opts {
            ips_or_hosts: vec!["127.0.0.1".to_owned(), "im_wrong".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 10,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let ips = parse_ips(&opts);

        assert_eq!(1, ips.len());
    }

    #[test]
    fn parse_incorrect_ips_or_hosts() {
        let opts = Opts {
            ips_or_hosts: vec!["im_wrong".to_owned(), "300.10.1.1".to_owned()],
            ports: None,
            range: None,
            quiet: true,
            batch_size: 10,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
            accessible: false,
            scan_order: ScanOrder::Serial,
        };
        let ips = parse_ips(&opts);

        assert_eq!(0, ips.len());
    }
}
