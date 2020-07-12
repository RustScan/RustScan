use colored::*;
use clap::{App, crate_authors};

fn main() {
    let _matches = App::new("RustScan")
        .author(crate_authors!())
        .about("Fast Port Scanner built in Rust")
        .version("0.01")
    
        .get_matches();
    print_opening()
    }
fn print_opening(){
    let s = "
    _____           _    _____                                 
    |  __ \\         | |  / ____|                                
    | |__) _   _ ___| |_| (___   ___ __ _ _ __  _ __   ___ _ __ 
    |  _  | | | / __| __|\\___ \\ / __/ _` | '_ \\| '_ \\ / _ | '__|
    | | \\ | |_| \\__ | |_ ____) | (_| (_| | | | | | | |  __| |   
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|_| |_|\\___|_|  "; 
        println!("{} \n {} \n {}", s.green(), "Automated Decryption Tool - https://github.com/ciphey/ciphey".red(),"Creator https://github.com/brandonskerritt".green());
    
}
