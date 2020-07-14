use colored::*;
use clap::{App, crate_authors, Arg};
use std::{str::FromStr, net::{IpAddr, TcpStream, SocketAddr}, ops::Range, u16, io};
use rayon::{current_num_threads, prelude::*};
use arrayvec::ArrayVec;
use std::time::Duration;

/// Faster Nmap scanning with Rust
///
fn main() {
    let vals = 0..100;

    let matches = App::new("RustScan")
        .author(crate_authors!())
        .about("Fast Port Scanner built in Rust")
        .version("0.01")

        // IP address is a required argument
        .arg(Arg::with_name("i")
            .required(true)
            .index(1)
            .long("--ip-address")
            .help("The IP address to scan"))
        .arg(Arg::with_name("p")
            .index(2)
            .long("--ports")
            .help("The port range you want to scan"))
        .get_matches();

    print_opening();

    let ip = matches.value_of("i").unwrap_or("None");

    if ip == "None"{
        println!("{}", "Error: No input was given.".red());
        return ();
    }

    // validatses the IP address and turns it into an IpAddr type
    let addr = IpAddr::from_str(&ip)
        .expect("IPADDR must be a valid IPv4 or IPv6 address");

    println!("IP is {}", addr);

    rayon::ThreadPoolBuilder::new().num_threads(1000).build_global().unwrap();
    thread_scan(addr);

    // let _nmap: &str = "nmap -A -sV -vvv -p $ports $ipaddr"
}

/// Runs Rayon to paralleise the scan

fn thread_scan(addr: IpAddr){
    
    // timeout in miliseconds
    // TODO set this to ping
    let duration_timeout = Duration::from_millis(600);

    // performs the scan using rayon
    // 65535 + 1 because of 0 indexing
    (1..65536).into_par_iter().for_each(|x: i32| {
        let string_list = vec![addr.to_string(), x.to_string()].join(":");
        let server: SocketAddr = string_list
        .parse()
        .expect("Unable to parse socket address");
        scan(server, duration_timeout);
        }
    )
}
    

fn scan(server: SocketAddr, duration_timeout: Duration){
    // pings it to see if its open
    //     match TcpStream::connect_timeout(&server, duration_timeout) {

    match TcpStream::connect_timeout(&server,duration_timeout) {
        Ok(_) => {
            // Found open port, indicate progress and send to main thread
            println!("{}", server.to_string().green());
        }
        Err(_) => {}

    }
}
fn print_opening(){
    let s = "
     _____           _    _____                 
    |  __ \\         | |  / ____|                
    | |__) |   _ ___| |_| (___   ___ __ _ _ __  
    |  _  / | | / __| __|\\___ \\ / __/ _` | '_ \\ 
    | | \\ \\ |_| \\__ \\ |_ ____) | (_| (_| | | | |
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|
    Faster nmap scanning with rust."; 
        println!("{} \n {} \n {}", s.green(), "Automated Decryption Tool - https://github.com/ciphey/ciphey".red(),"Creator https://github.com/brandonskerritt".green());
    
}

