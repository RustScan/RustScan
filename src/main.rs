use colored::*;
use clap::{App, crate_authors, Arg};

fn main() {
    let _matches = App::new("RustScan")
        .author(crate_authors!())
        .about("Fast Port Scanner built in Rust")
        .version("0.01")
        .arg(Arg::with_name("i")
            .help("The IP address to scan"))
        .arg(Arg::with_name("p")
            .help("The port range you want to scan"))
        .get_matches();
    print_opening()
    
    // let _nmap: &str = "nmap -A -sV -vvv -p $ports $ipaddr"
}
fn print_opening(){
    let s = "
     _____           _    _____                 
    |  __ \\         | |  / ____|                
    | |__) |   _ ___| |_| (___   ___ __ _ _ __  
    |  _  / | | / __| __|\\___ \\ / __/ _` | '_ \\ 
    | | \\ \\ |_| \\__ \\ |_ ____) | (_| (_| | | | |
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|"; 
        println!("{} \n {} \n {}", s.green(), "Automated Decryption Tool - https://github.com/ciphey/ciphey".red(),"Creator https://github.com/brandonskerritt".green());
    
}
