use colored::*;
use clap::{App, crate_authors, Arg};
use std::{str::FromStr, net::{IpAddr, TcpStream}, ops::Range, u16, io};
use rayon::prelude::*;
use arrayvec::ArrayVec;

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
    // performs the scan using rayon
    for x in (80..81){
        scan(addr, x);
    }
    //et ports = (1..65535).into_par_iter().for_each::<_>(|port: u16| scan(addr, port));
}

fn scan(ip: IpAddr, start_port: u16){
    // ports is a list slice of X ports
    // This depends on the threads useud
    // Usually around ~5 ports
    // 
    match TcpStream::connect((ip, start_port)) {
        Ok(_) => {
            // Found open port, indicate progress and send to main thread
            print!("Found ip {} and port {}", ip, start_port);
        }
        Err(_) => {}

}}
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

