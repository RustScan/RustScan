#!/bin/sh
//usr/bin/env rustc $0 -o a.out && ./a.out && rm ./a.out ; exit
use async_std::io;
use async_std::net::TcpStream;
use clap::{App, Arg, AppSettings};
use colored::*;
use std::process::{exit, Command};
use std::time::Duration;
use std::{
    net::{SocketAddr, Shutdown},
    io::ErrorKind,
};
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use futures::executor::block_on;
/// Faster Nmap scanning with Rust
fn main() {
    let matches = App::new("RustScan")
        .author("Bee https://github.com/brandonskerritt")
        .about("Fast Port Scanner built in Rust")
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
        .arg(
            Arg::with_name("command")
                .help("The Nmap arguments to run. To use the argument -A, end RustScan's args with '-- -A'. To run EXAMPLE: 'rustscan -T 1500 127.0.0.1 -- -A -sC'. This argument auto runs nmap {your commands} -vvv -p $PORTS ")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    print_opening();

    let ip = matches.value_of("ip").unwrap_or("None");
    let command_matches= matches.values_of("command");
    let command_run: String = match command_matches {
        // We use the user supplied args
        Some(x) => {
            // TODO x is the same as below, use that instead
            matches.values_of("command").unwrap().collect::<Vec<_>>().join(" ")
        }
        // we default
        None    => "-A -vvv".to_string()

    };

    let batch_size: u32 = matches
                        .value_of("b")
                        .unwrap_or("None")
                        .parse::<u32>()
                        .unwrap();
                            
    // gets timeout
    let duration_timeout =
        matches
            .value_of("T")
            .unwrap_or("None")
            .parse::<u64>()
            .unwrap();


    // 65535 + 1 because of 0 indexing
    let test = run_batched(ip.to_string(), 1, 65536, Duration::from_millis(duration_timeout),  batch_size);
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
    println!(
        "{}",
        "Starting nmap.".blue(),
    );

    // nmap port style is 80,443. Comma seperated with no spaces.
    let ports_str = nmap_str_ports.join(",");
    let string_format = format!("{} {} {} {} {}", command_run, "-vvv", "-p", &ports_str, ip);
    let command_list = string_format.split_whitespace();
    let vec = command_list.collect::<Vec<&str>>();

    // Runs the nmap command and spawns it as a process.
    Command::new("nmap")
        .args(&vec)
        .spawn()
        .expect("failed to execute process");
}

pub async fn run_batched(
    host: String,
    port_start: u32,
    port_end: u32,
    timeout: Duration,
    batch: u32,
) -> Vec<u32> {
    // run the scans in batches
    let mut begin = port_start;
    let mut end = begin + batch;
    let mut all_addrs: std::vec::Vec<u32> = Vec::new();

    while end <= port_end {
        let mut batch_addrs = execute(host.clone(), begin, end, timeout).await;
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
) -> Vec<u32> {
    // runs a scan against a range of ports
    let mut ftrs = FuturesUnordered::new();
    // TODO can I make this async?
    for port in port_start..port_end {
        ftrs.push(try_connect(host.clone(), port, timeout));
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

async fn try_connect(host: String, port: u32, timeout: Duration) -> io::Result<u32> {
    let addr = host.to_string() + ":" + &port.to_string();
    match addr.parse() {
        Ok(sock_addr) => match connect(sock_addr, timeout).await {
            Ok(stream_result) => {
                match stream_result.shutdown(Shutdown::Both) {
                    _ => {}
                }
                println!("Open {}", port.to_string().purple());
                Ok(port)
            }
            Err(e) => match e.kind() {
                ErrorKind::Other => {
                    eprintln!("{:?}", e); // in case we get too many open files
                    panic!("Too many open files. Please reduce batch size. The default is 5000. Try -B 2500.");
                    Err(e)
                }
                _ => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        },
        Err(e) => {
            eprintln!("Unable to convert to socket address {:?}", e);
            panic!("Unable to convert to socket address");
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
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
        "Automated Decryption Tool - https://github.com/ciphey/ciphey".red(),
        "Creator https://github.com/brandonskerritt".green()
    );
}
