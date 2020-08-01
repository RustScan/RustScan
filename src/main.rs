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

#[macro_use]
extern crate log;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// Fast Port Scanner built in Rust.
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
struct Opts {
    /// The IP address to scan
    #[structopt(parse(try_from_str))]
    ip: IpAddr,

    ///Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    quiet: bool,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    batch_size: u64,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    timeout: u64,

    /// Automatically ups the ULIMIT with the value you provided.
    #[structopt(short, long)]
    ulimit: Option<u64>,

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
    let mut opts = Opts::from_args();
    info!("Mains() `opts` arguments are {:?}", opts);

    if !opts.quiet {
        print_opening();
    }

    // Updates ulimit when the argument is set

    // Automatically ups the ulimit
    if opts.ulimit.is_some() {
        let limit = opts.ulimit.unwrap();
        info!("Automatically upping ulimit");

        if !opts.quiet {
            println!("Automatically upping ulimit to {}", limit);
        }

        match setrlimit(Resource::NOFILE, limit, limit) {
            Ok(_) => {}
            Err(_) => println!("ERROR.  Failed to set Ulimit."),
        }
    }

    let (x, _) = getrlimit(Resource::NOFILE).unwrap();

    // if maximum limit is lower than batch size
    // automatically re-adjust the batch size
    if x < opts.batch_size {
        if !opts.quiet {
            println!("{}", "WARNING: Your file description limit is lower than selected batch size. Please considering upping this (how to is on the README). NOTE: this may be dangerous and may cause harm to sensitive servers. Automatically reducing Batch Size to match your limit, this process isn't harmful but reduces speed.".red());
        }

        // if the OS supports high file limits like 8000
        // but the user selected a batch size higher than this
        // reduce to a lower number
        // basically, ubuntu is 8000
        // but i can only get it to work on < 5k in testing
        // 5k is default, so 3000 seems safe
        if x > 8000 {
            opts.batch_size = 3000
        } else {
            opts.batch_size = x - 100u64
        }
    }
    // else if the ulimit is higher than batch size
    // tell the user they can increase batch size
    // if the user set ulimit arg they probably know what they are doing so don't print this
    else if x + 2 > opts.batch_size.into() && (opts.ulimit.is_none()) {
        if !opts.quiet {
            println!(
                "Your file description limit is higher than the batch size. You can potentially increase the speed by increasing the batch size, but this may cause harm to sensitive servers. Your limit is {}, try batch size {}.",
                x,
                x - 1u64
            );
        }
    }

    // 65535 + 1 because of 0 indexing
    let scanner = Scanner::new(
        opts.ip,
        1,
        65535,
        opts.batch_size.into(),
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

    let addr = opts.ip.to_string();
    let user_nmap_args =
        shell_words::split(&opts.command.join(" ")).expect("failed to parse nmap arguments");
    let nmap_args = build_nmap_arguments(&addr, &ports_str, &user_nmap_args, opts.ip.is_ipv6());

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
        let scanner = Scanner::new(addr, 1, 65535, 1000, Duration::from_millis(10), true);
        let scan_result = block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    fn does_it_run_ipv6() {
        // Makes sure te program still runs and doesn't panic
        let addr = match "::1".parse::<IpAddr>() {
            Ok(res) => res,
            Err(_) => panic!("Could not parse IP Address"),
        };
        let scanner = Scanner::new(addr, 1, 65535, 1000, Duration::from_millis(10), true);
        let scan_result = block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
}
