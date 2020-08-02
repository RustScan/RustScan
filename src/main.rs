extern crate shell_words;

mod scanner;
use scanner::Scanner;

use colored::*;
use futures::executor::block_on;
use rlimit::Resource;
use rlimit::{getrlimit, setrlimit};
use std::process::{exit, Command};
use std::{net::IpAddr, time::Duration};
use structopt::StructOpt;

extern crate dirs;
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
    /// The IP address to scan
    #[structopt(parse(try_from_str))]
    ip: Option<IpAddr>,

    ///Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    quiet: bool,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    batch_size: u32,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    timeout: u32,

    /// Automatically ups the ULIMIT with the value you provided.
    #[structopt(short, long)]
    ulimit: Option<rlimit::rlim>,

    // Appdirs location. Use this to print out where the config file should go.
    #[structopt(short, long)]
    appdirs: bool,

    /// The Nmap arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -T 1500 127.0.0.1 -- -A -sC'.
    /// This command adds -Pn -vvv -p $PORTS automatically to nmap.
    /// For things like --script '(safe and vuln)' enclose it in quotations marks \"'(safe and vuln)'\"")
    command: Vec<String>,
}

/// Faster Nmap scanning with Rust
/// If you're looking for the actual scanning, check out the module Scanner.
fn main() {
    // logger
    env_logger::init();

    info!("Starting up");
    let opts = Opts::from_args();
    info!("Mains() `opts` arguments are {:?}", opts);

    let config = dirs::config_dir();

    let mut config_path = match config {
        Some(x) => x,
        None => panic!("Couldn't find config dir."),
    };
    config_path.push("rustscan");
    config_path.push("config.toml");

    if opts.appdirs {
        // prints config file location and exits
        println!("The config file is expected to be at {:?}", config_path);
        exit(1);
    }

    let ip = match opts.ip {
        Some(ip) => ip,
        None => panic!("Error. No IP address was supplied."),
    };

    if !opts.quiet {
        print_opening();
    }

    let ulimit: rlimit::rlim = adjust_ulimit_size(&opts);
    let batch_size: u32 = infer_batch_size(&opts, ulimit);

    // 65535 + 1 because of 0 indexing
    let scanner = Scanner::new(
        ip,
        1,
        65535,
        batch_size,
        Duration::from_millis(opts.timeout.into()),
        opts.quiet,
    );
    let scan_result = block_on(scanner.run());

    // prints ports and places them into nmap string
    let nmap_str_ports: Vec<String> = scan_result
        .into_iter()
        .map(|port| port.to_string())
        .collect();

    // if no ports are found, suggest running with less
    if nmap_str_ports.is_empty() {
        panic!("{} Looks like I didn't find any open ports. This is usually caused by a high batch size.
        \n*I used {} batch size, consider lowering to {} with {} or a comfortable number for your system.
        \n Alternatively, increase the timeout if your ping is high. Rustscan -T 2000 for 2000 second timeout.", "ERROR".red(),
        opts.batch_size,
        (opts.batch_size / 2).to_string().green(),
        "'rustscan -b <batch_size> <ip address>'".green());
    }

    // Tells the user we are now switching to Nmap
    if !opts.quiet {
        println!("{}", "Starting nmap.".blue(),);
    }

    // nmap port style is 80,443. Comma seperated with no spaces.
    let ports_str = nmap_str_ports.join(",");

    // if quiet mode is on, return ports and exit
    if opts.quiet {
        println!("{}", ports_str);
        exit(1);
    }

    let addr = ip.to_string();
    let user_nmap_args =
        shell_words::split(&opts.command.join(" ")).expect("failed to parse nmap arguments");
    let nmap_args = build_nmap_arguments(&addr, &ports_str, &user_nmap_args, ip.is_ipv6());

    if !opts.quiet {
        println!("The Nmap command to be run is {}", &nmap_args.join(" "));
    }

    // Runs the nmap command and spawns it as a process.
    let mut child = Command::new("nmap")
        .args(&nmap_args)
        .spawn()
        .expect("failed to execute nmap process");

    child.wait().expect("failed to wait on nmap process");
}

/// Prints the opening title of RustScan
fn print_opening() {
    info!("Printing opening");
    let s = "
     _____           _    _____
    |  __ \\         | |  / ____|
    | |__) |   _ ___| |_| (___   ___ __ _ _ __
    |  _  / | | / __| __|\\___ \\ / __/ _` | '_ \\
    | | \\ \\ |_| \\__ \\ |_ ____) | (_| (_| | | | |
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|
    Faster nmap scanning with rust.";
    println!("{}\n", s.green());
}

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
                    println!("\nAutomatically increasing ulimit value to {}.\n", limit);
                }
            }
            Err(_) => println!("{}", "ERROR. Failed to set ulimit value.".red()),
        }
    }

    let (rlim, _) = getrlimit(Resource::NOFILE).unwrap();

    rlim
}

fn infer_batch_size(opts: &Opts, ulimit: rlimit::rlim) -> u32 {
    let mut batch_size: rlimit::rlim = opts.batch_size.into();

    // Adjust the batch size when the ulimit value is lower than the desired batch size
    if ulimit < batch_size {
        if !opts.quiet {
            println!("{}", "WARNING: Your file description limit is lower than the provided batch size. Please considering upping this (instructions in our README). NOTE: this may be dangerous and may cause harm to sensitive servers. Automatically reducing the batch Size to match your system's limit, this process isn't harmful but reduces speed.".red());
        }

        // When the OS supports high file limits like 8000, but the user
        // selected a batch size higher than this we should reduce it to
        // a lower number.
        if ulimit > DEFAULT_FILE_DESCRIPTORS_LIMIT && ulimit > AVERAGE_BATCH_SIZE {
            // if ulimt is more than the default && the average size on Ubuntu
            // the user has a weird OS with an incredibly small ulimit
            // so we half it to prevent any weird errors propping up because of it.
            batch_size = ulimit / 2
        } else if ulimit > DEFAULT_FILE_DESCRIPTORS_LIMIT {
            batch_size = AVERAGE_BATCH_SIZE
        } else {
            batch_size = ulimit - 100
        }
    }
    // When the ulimit is higher than the batch size let the user know that the
    // batch size can be increased unless they specified the ulimit themselves.
    else if ulimit + 2 > batch_size && (opts.ulimit.is_none()) {
        if !opts.quiet {
            println!(
                "Your file descriptor limit is higher than the batch size. You can potentially increase the speed by increasing the batch size, but this may cause harm to sensitive servers. Your limit is {}, try batch size {}.",
                ulimit,
                ulimit - 1
            );
        }
    }

    batch_size as u32
}

#[cfg(test)]
mod tests {
    use super::Scanner;
    use async_std::task::block_on;
    use std::{net::IpAddr, time::Duration};

    #[test]
    fn does_it_run() {
        // Makes sure te program still runs and doesn't panic
        let addr = match "127.0.0.1".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 1, 65535, 100, Duration::from_millis(10), true);
        let scan_result = block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[test]
    fn does_it_run_ipv6() {
        // Makes sure te program still runs and doesn't panic
        let addr = match "::1".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 1, 65535, 100, Duration::from_millis(10), true);
        let scan_result = block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[test]
    fn does_it_run_cmnatic() {
        // Makes sure te program still runs and doesn't panic
        // this IP was donated to us for use by Cmnatic
        let addr = match "51.140.231.146".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 5000, 250, 500, Duration::from_millis(1000), true);
        let scan_result = block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(true, scan_result.contains(&5678));
    }
    #[test]
    fn does_it_run_triple_0() {
        let addr = match "0.0.0.0".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 1, 65535, 10_000, Duration::from_millis(5), true);
        let scan_result = block_on(scanner.run());
        assert_eq!(1, 1);
    }
    #[test]
    fn zero_ports() {
        let addr = match "0.0.0.0".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 1, 1, 10_000, Duration::from_millis(5), true);
        let scan_result = block_on(scanner.run());
        assert_eq!(1, 1);
    }
    #[test]
    fn backwards_ports() {
        let addr = match "0.0.0.0".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 10, 1, 10_000, Duration::from_millis(5), true);
        let scan_result = block_on(scanner.run());
        assert_eq!(1, 1);
    }
    #[test]
    fn google_test() {
        let addr = match "8.8.8.8".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 10, 1, 10_000, Duration::from_millis(1500), true);
        let scan_result = block_on(scanner.run());
        assert_eq!(1, 1);
    }
}
