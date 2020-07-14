use colored::*;
use clap::{App, crate_authors, Arg};
use std::{str::FromStr, net::{IpAddr, TcpStream, SocketAddr}, ops::Range, u16, io};
use rayon::{current_num_threads, prelude::*};
use arrayvec::ArrayVec;
use std::time::Duration;

// Upper Port Limit
const NUM: u32 = 65535;
// The default config for users
struct Config {
    ip: IpAddr,
    nmap_command: String,
    ports: [i32],
}

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

    // get the value and store in a1_val
    //if let Some(ip) = matches.value_of("i"){
     //   println!("{}", ip);
    //}

    if ip == "None"{
        println!("{}", "Error: No input was given.".red());
        return ();
    }

    // validatses the IP address and turns it into an IpAddr type
    let addr = IpAddr::from_str(&ip)
        .expect("IPADDR must be a valid IPv4 or IPv6 address");

    println!("IP is {}", addr);

    // if ports not specified, use this:

    // creates array of ports up to max port num
    /*let mut ports: [i32; 65535] = [0; NUM];
    for (i, v) in ports.iter_mut().enumerate() {
        *v = i as i32
    }*/

    thread_scan(addr);
    // println!("{}. {}", addr, port);
    // scan(addr, port)


    //scan(addr, ports)
    
    
    // let _nmap: &str = "nmap -A -sV -vvv -p $ports $ipaddr"
}

/// Runs Rayon to paralleise the scan

fn thread_scan(addr: IpAddr){
    // (IP, _)
    
    // timeout in miliseconds
    // TODO set this to ping
    let duration_timeout = Duration::from_millis(100);

    // performs the scan using rayon
    (1..1000).into_par_iter().for_each(|x: i32| {
        let string_list = vec![addr.to_string(), x.to_string()].join(":");
        let server: SocketAddr = string_list
        .parse()
        .expect("Unable to parse socket address");
        println!("{}", current_num_threads());
        scan(server, duration_timeout);
    })
    
    /*for x in (1..83){
        scan(addr, x, duration_timeout);
    }*/
    //et ports = (1..65535).into_par_iter().for_each::<_>(|port: u16| scan(addr, port));
}

fn scan(server: SocketAddr, duration_timeout: Duration){
    println!("Running scan");
    // ports is a list slice of X ports
    // This depends on the threads useud
    // Usually around ~5 ports
    // #

    // TODO move this out to thread_scan
    // TODO Please speed up this code
    // makes the ip + port number

    // pings it to see if its open
    match TcpStream::connect_timeout(&server, duration_timeout) {
        Ok(_) => {
            // Found open port, indicate progress and send to main thread
            print!("{}", server);
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

