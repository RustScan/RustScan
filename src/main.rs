extern crate shell_words;

mod tui;

mod scanner;
use scanner::Scanner;

use colorful::Color;
use colorful::Colorful;
use futures::executor::block_on;
use rlimit::Resource;
use rlimit::{getrlimit, setrlimit};
use std::collections::HashMap;
use std::process::Command;
use std::{net::IpAddr, time::Duration};
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

#[derive(StructOpt, Debug)]
#[structopt(name = "rustscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// Fast Port Scanner built in Rust.
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
/// - Discord https://discord.gg/GFrQsGy
/// - GitHub https://github.com/RustScan/RustScan
struct Opts {
    /// A list of comma separated IP addresses to be scanned.
    #[structopt(use_delimiter = true, parse(try_from_str), required = true)]
    ips: Vec<IpAddr>,

    ///Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    quiet: bool,

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

    /// The Nmap arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -T 1500 127.0.0.1 -- -A -sC'.
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
    let opts = Opts::from_args();
    info!("Mains() `opts` arguments are {:?}", opts);

    if !opts.quiet {
        print_opening();
    }

    let ulimit: rlimit::rlim = adjust_ulimit_size(&opts);
    let batch_size: u16 = infer_batch_size(&opts, ulimit);

    let scanner = Scanner::new(
        &opts.ips,
        LOWEST_PORT_NUMBER,
        TOP_PORT_NUMBER,
        batch_size,
        Duration::from_millis(opts.timeout.into()),
        opts.quiet,
    );

    let scan_result = block_on(scanner.run());
    let mut ports_per_ip = HashMap::new();

    for socket in scan_result {
        ports_per_ip
            .entry(socket.ip())
            .or_insert_with(Vec::new)
            .push(socket.port());
    }

    for ip in opts.ips {
        if ports_per_ip.contains_key(&ip) {
            continue;
        }

        // If we got here it means the IP was not found within the HashMap, this
        // means the scan couldn't find any open ports for it.
        if opts.quiet {
            let x = format!("{} {:?}", "No ports found for", ip);
            detail!(x);
        } else {
            let x = format!("{} Looks like I didn't find any open ports for {:?}. This is usually caused by a high batch size.
            \n*I used {} batch size, consider lowering to {} with {} or a comfortable number for your system.
            \n Alternatively, increase the timeout if your ping is high. Rustscan -T 2000 for 2000 second timeout.\n",
            "ERROR",
            ip,
            opts.batch_size,
            (opts.batch_size / 2).to_string(),
            "'rustscan -b <batch_size> <ip address>'");
            warning!(x);
        }
    }

    for (ip, ports) in ports_per_ip.iter_mut() {
        let nmap_str_ports: Vec<String> = ports.into_iter().map(|port| port.to_string()).collect();

        if !opts.quiet {
            println!("\n");
            detail!("Starting Nmap\n");
        }

        // nmap port style is 80,443. Comma separated with no spaces.
        let ports_str = nmap_str_ports.join(",");

        // if quiet mode is on nmap should not be spawned
        if opts.quiet {
            output!(format!("Ports: {:?}", ports_str));
            continue;
        }

        let addr = ip.to_string();
        let user_nmap_args =
            shell_words::split(&opts.command.join(" ")).expect("failed to parse nmap arguments");
        let nmap_args = build_nmap_arguments(&addr, &ports_str, &user_nmap_args, ip.is_ipv6());

        output!(format!(
            "The Nmap command to be run is {}",
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
Faster Nmap scanning with Rust.
Discord: https://discord.gg/GFrQsGy
GitHub: https://github.com/RustScan/RustScan
🌍HACK THE PLANET🌍"#;
    println!("{}\n", s.gradient(Color::Green));

    let config_path = match dirs::config_dir() {
        Some(mut path) => {
            path.push("rustscan");
            path.push("config.toml");
            path
        }
        None => panic!("Couldn't find config dir."),
    };

    detail!(format!(
        "{} {:?}\n",
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
    arguments.push("-A");
    arguments.push("-vvv");

    if is_ipv6 {
        arguments.push("-6");
    }

    arguments.push("-p");
    arguments.push(ports);
    arguments.push(addr);

    arguments
}

fn adjust_ulimit_size(opts: &Opts) -> rlimit::rlim {
    if opts.ulimit.is_some() {
        let limit: rlimit::rlim = opts.ulimit.unwrap();

        match setrlimit(Resource::NOFILE, limit, limit) {
            Ok(_) => {
                if !opts.quiet {
                    detail!(format!(
                        "Automatically increasing ulimit value to {}.",
                        limit
                    ));
                }
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
        if !opts.quiet {
            warning!("File limit is lower than default batch size. Consider upping with --ulimt. May cause harm to sensitive servers");
        }

        // When the OS supports high file limits like 8000, but the user
        // selected a batch size higher than this we should reduce it to
        // a lower number.
        if ulimit < AVERAGE_BATCH_SIZE {
            // ulimit is smaller than aveage batch size
            // user must have very small ulimit
            // decrease batch size to half of ulimit
            warning!("Your file limit is very small, which negatively impacts RustScan's speed. Use the Docker image, or up the Ulimt with '--ulimt 5000'. ");
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
        if !opts.quiet {
            detail!(format!(
                "File limit higher than batch size. Can gain speed by increasing batch size '-b {}'.",
                ulimit - 100
            ));
        }
    }

    batch_size as u16
}

#[cfg(test)]
mod tests {
    use crate::{adjust_ulimit_size, infer_batch_size, print_opening, Opts};
    use std::{net::IpAddr, str::FromStr};

    #[test]
    fn batch_size_lowered() {
        let opts = Opts {
            ips: vec![IpAddr::from_str("127.0.0.1").unwrap()],
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
        };
        let batch_size = infer_batch_size(&opts, 120);

        assert!(batch_size < 50_000);
    }

    #[test]
    fn batch_size_lowered_average_size() {
        let opts = Opts {
            ips: vec![IpAddr::from_str("127.0.0.1").unwrap()],
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
        };
        let batch_size = infer_batch_size(&opts, 9_000);

        assert!(batch_size == 3_000);
    }
    #[test]
    fn batch_size_equals_ulimit_lowered() {
        // because ulimit and batch size are same size, batch size is lowered
        // to ULIMIT - 100
        let opts = Opts {
            ips: vec![IpAddr::from_str("127.0.0.1").unwrap()],
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
        };
        let batch_size = infer_batch_size(&opts, 5_000);

        assert!(batch_size == 4_900);
    }
    #[test]
    fn batch_size_adjusted_2000() {
        // ulimit == batch_size
        let opts = Opts {
            ips: vec![IpAddr::from_str("127.0.0.1").unwrap()],
            quiet: true,
            batch_size: 50_000,
            timeout: 1_000,
            ulimit: Some(2_000),
            command: Vec::new(),
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
            ips: vec![IpAddr::from_str("127.0.0.1").unwrap()],
            quiet: false,
            batch_size: 10,
            timeout: 1_000,
            ulimit: None,
            command: Vec::new(),
        };

        infer_batch_size(&opts, 1_000_000);

        assert!(1 == 1);
    }
}
