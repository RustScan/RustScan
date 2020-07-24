use async_std::io;
use async_std::net::TcpStream;
use clap::{App, Arg, AppSettings};
use colored::*;
use std::process::{exit, Command};
use std::time::Duration;
use std::{
    net::{SocketAddr, Shutdown},
    io::ErrorKind, convert::TryInto,
};
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use futures::executor::block_on;
use rlimit::Resource;
use rlimit::{setrlimit, getrlimit};
/// Faster Nmap scanning with Rust
fn main() {
    let matches = App::new("RustScan")
        .author("Bee https://github.com/brandonskerritt")
        .about("Fast Port Scanner built in Rust\nWARNING Do not use this program against sensitive infrastructure. The specified server may not be able to handle this many socket connections at once.")
        .version("1.2.0")
        .setting(AppSettings::TrailingVarArg)
        
        // IP address is a required argument
        .arg(Arg::with_name("ip")
            .required(true)
            .index(1)
            .long("ip-address")
            .help("The IP address to scan"))
        .arg(Arg::with_name("b")
            .short("b")
            .long("batch")
            .takes_value(true)
            .default_value("4500")
            .help("Increases speed of scanning. The batch size for port scanning. Depends on your open file limit of OS. If you do 65535 it will do every port at the same time. Although, your OS may not support this."))
        .arg(Arg::with_name("T")
            .short("T")
            .long("timeout")
            .takes_value(true)
            .default_value("1500")
            .help("The timeout before a port is assumed to be close. In MS."))
        .arg(Arg::with_name("q")
            .short("-q")
            .long("quiet")
            .takes_value(false)
            .help("Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file."))
        .arg(Arg::with_name("u")
            .short("u")
            .long("ulimit")
            .help("Automatically ups the ULIMIT with the value you provided.")
            .takes_value(true))
        .arg(
            Arg::with_name("command")
                .help("The Nmap arguments to run. To use the argument -A, end RustScan's args with '-- -A'. To run EXAMPLE: 'rustscan -T 1500 127.0.0.1 -- -A -sC'. This argument auto runs nmap {your commands} -vvv -p $PORTS ")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    
    let ip = matches.value_of("ip").unwrap_or("None");
    let ulimit_arg = matches.value_of("u").unwrap_or("None");
    let quiet = if matches.is_present("q") { true } else { false };
    let command_matches= matches.values_of("command");
    let command_run: String = match command_matches {
        // We use the user supplied args
        Some(_x) => {
            // TODO x is the same as below, use that instead
            matches.values_of("command").unwrap().collect::<Vec<_>>().join(" ")
        }
        // we default
        None    => "-A -vvv".to_string()

    };

    let mut batch_size: u64 = matches
                        .value_of("b")
                        .unwrap_or("None")
                        .parse::<u64>()
                        .unwrap();
    
    if !quiet {
        print_opening();
    }
    
    // checks ulimit
    
    // change ulimit size
    if !(ulimit_arg == "None") {
        let limit = ulimit_arg.parse::<u64>().unwrap();
        if !quiet{
            println!("Automatically upping ulimit to {}", ulimit_arg);
        }
        let uresult = setrlimit(Resource::NOFILE, limit, limit);

        match uresult {
            Ok(_) => {}
            Err(_) => {println!("ERROR.  Failed to set Ulimit.")}
        }
    }

    

    let (x, _) = getrlimit(Resource::NOFILE).unwrap(); 

    // if maximum limit is lower than batch size
    // automatically re-adjust the batch size
    if x < batch_size.into() {
        if !quiet{
            println!("{}", "WARNING: Your file description limit is lower than selected batch size. Please considering upping this (how to is on the README). NOTE: this may be dangerous and may cause harm to sensitive servers. Automatically reducing Batch Size to match your limit, this process isn't harmful but reduces speed.".red());
            // TODO this is a joke please fix

            // if the OS supports high file limits like 8000
            // but the user selected a batch size higher than this
            // reduce to a lower number
            // basically, ubuntu is 8000 
            // but i can only get it to work on < 5k in testing
            // 5k is default, so 3000 seems safe
            if x > 8000{
                batch_size = 3000
            }
            else {
                let ten: u64 = 100;
                batch_size = x - ten;
            }
        }
    }
    // else if the ulimit is higher than batch size
    // tell the user they can increase batch size
    // if the user set ulimit arg they probably know what they are doing so don't print this
    else if x + 2 > batch_size.into() && (ulimit_arg == "None"){
        if !quiet{
            // TODO this is a joke please fix
            let one: u64 = 1;
            println!("Your file description limit is higher than the batch size. You can potentially increase the speed by increasing the batch size, but this may cause harm to sensitive servers. Your limit is {}, try batch size {}.", x, x - one);
        }
    }
    // the user has asked to automatically up the ulimit

                            
    // gets timeout
    let duration_timeout =
        matches
            .value_of("T")
            .unwrap_or("None")
            .parse::<u64>()
            .unwrap();



    // 65535 + 1 because of 0 indexing
    let test = run_batched(ip.to_string(), 1, 65536, Duration::from_millis(duration_timeout,),  batch_size.try_into().unwrap(), quiet);
    let reports_fullsult = block_on(test);


    // prints ports and places them into nmap string
    let mut nmap_str_ports: Vec<String> = Vec::new();

    // makes vector of open ports
    for i in reports_fullsult.iter() {
            // appends it to port
            nmap_str_ports.push(i.to_string());
    }

    // if no ports are found, suggest running with less 
    if nmap_str_ports.is_empty() {
        panic!("{} Looks like I didn't find any open ports. This is usually caused by a high batch size.
        \n*I used {} threads, consider lowering to {} with {} or a comfortable number lfor your system. 
        \n Alternatively, increase the timeout if your ping is high. Rustscan -T 2000 for 2000 second timeout.", "ERROR".red(), batch_size, (batch_size / 2).to_string().green(), "'rustscan -b <batch_size> <ip address>'".green());
    }

    // Tells the user we are now switching to Nmap
    if !quiet{
        println!(
            "{}",
            "Starting nmap.".blue(),
        );
    }


    // nmap port style is 80,443. Comma seperated with no spaces.
    let ports_str = nmap_str_ports.join(",");

    // if quiet mode is on, return ports and exit
    if quiet{
        println!("{:?}", ports_str);
        exit(1);
    }

    let string_format = format!("{} {} {} {} {}", command_run, "-vvv", "-p", &ports_str, ip);
    let command_list = string_format.split_whitespace();
    let vec = command_list.collect::<Vec<&str>>();

    // Runs the nmap command and spawns it as a process.
    let mut child = Command::new("nmap")
        .args(&vec)
        .spawn()
        .expect("failed to execute nmap process");

    child.wait().expect("failed to wait on nmap process");
}

pub async fn run_batched(
    host: String,
    port_start: u32,
    port_end: u32,
    timeout: Duration,
    batch: u32,
    quiet: bool,
) -> Vec<u32> {
    // run the scans in batches
    let mut begin = port_start;
    let mut end = begin + batch;
    let mut all_addrs: std::vec::Vec<u32> = Vec::new();

    while end <= port_end {
        let mut batch_addrs = execute(host.clone(), begin, end, timeout, quiet).await;
        all_addrs.append(&mut batch_addrs);
        begin = end+1;
        end += batch;
    }
    all_addrs
}
async fn execute(
    host: String,
    port_start: u32,
    port_end: u32,
    timeout: Duration,
    quiet: bool,
) -> Vec<u32> {
    // runs a scan against a range of ports
    let mut ftrs = FuturesUnordered::new();
    // TODO can I make this async?
    for port in port_start..port_end {
        ftrs.push(try_connect(host.clone(), port, timeout, quiet));
    }

    let mut open_addrs: Vec<u32> = Vec::new();
    // TODO can I make this async?
    while let Some(result) = ftrs.next().await {
        match result {
            Ok(addr) => open_addrs.push(addr),
            Err(_) => {}
        }
    }
    open_addrs
}

async fn try_connect(host: String, port: u32, timeout: Duration, quiet: bool) -> io::Result<u32> {
    let addr = host.to_string() + ":" + &port.to_string();
    match addr.parse() {
        Ok(sock_addr) => match connect(sock_addr, timeout).await {
            Ok(stream_result) => {
                match stream_result.shutdown(Shutdown::Both) {
                    _ => {}
                }
                if !quiet {
                    println!("Open {}", port.to_string().purple());
                }
                
                Ok(port)
            }
            Err(e) => match e.kind() {
                ErrorKind::Other => {
                    eprintln!("{:?}", e); // in case we get too many open files
                    panic!("Too many open files. Please reduce batch size. The default is 5000. Try -b 2500.");
                }
                _ => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        },
        Err(e) => {
            eprintln!("Unable to convert to socket address {:?}", e);
            panic!("Unable to convert to socket address");
        }
    }
}


async fn connect(addr: SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
    let stream = io::timeout(timeout, async move { TcpStream::connect(addr).await }).await?;
    Ok(stream)
}

fn print_opening() {
    let s = "
     _____           _    _____                 
    |  __ \\         | |  / ____|                
    | |__) |   _ ___| |_| (___   ___ __ _ _ __  
    |  _  / | | / __| __|\\___ \\ / __/ _` | '_ \\ 
    | | \\ \\ |_| \\__ \\ |_ ____) | (_| (_| | | | |
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|
    Faster nmap scanning with rust.";
    println!(
        "{} \n {} \n {}",
        s.green(),
        "Automated Decryption Tool - https://github.com/ciphey/ciphey".green(),
        "Creator https://github.com/brandonskerritt".green()
    );
}
