use colored::*;
use clap::{App, crate_authors, Arg};
use std::net::{IpAddr, TcpStream};
use std::{str::FromStr, io};
// Upper Port Limit
const MAX: u16 = 65535;

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
            .help("The IP address to scan"))
        .arg(Arg::with_name("p")
            .help("The port range you want to scan"))
        .get_matches();
    print_opening();

    // validatses the IP address and turns it into an IpAddr type
    let addr = IpAddr::from_str(&matches.free[0])
        .expect("IPADDR must be a valid IPv4 or IPv6 address");

    // if ports not specified, use this:
    let ports = 0..MAX;
    
    
    // let _nmap: &str = "nmap -A -sV -vvv -p $ports $ipaddr"
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

fn scan(ip: IpAddr, start_port: u16){
    // ports is a list slice of X ports
    // This depends on the threads useud
    // Usually around ~5 ports
    // 
    match TcpStream::connect((ip, start_port)) {
        Ok(_) => {
            // Found open port, indicate progress and send to main thread
            print!("Found ip {} and port {}", ip, start_port);
            io::stdout().flush().unwrap();
        }
        Err(_) => {}

}
