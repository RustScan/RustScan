/// Terminal User Interface Module for RustScan
/// Defines macros to use

// TODO make them work with formatted strings

#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        use ansi_term::Colour::Red;
        if $name.len() > 200 {
            panic!("Your warning is too long.")
        }
        println!("{} {}", Red.bold().paint("[!]"), $name);
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        use ansi_term::Colour::Blue;
        if $name.len() > 200 {
            panic!("Your detail is too long.")
        }
        println!("{} {}", Blue.bold().paint("[~]"), $name);
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr) => {
        use ansi_term::Colour::RGB;

        println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
    };
}

#[macro_export]
macro_rules! funny_opening {
    // prints a funny quote / opening
    () => {
        use rand::seq::SliceRandom;
        let quotes = vec!["Nmap? More like slowmap.🐢", "🌍HACK THE PLANET🌍", "Real hackers hack time ⌛", "Please contribute more quotes to our GitHub https://github.com/rustscan/rustscan", 
        "😵 https://admin.tryhackme.com"];
        let random_quote = quotes.choose(&mut rand::thread_rng()).unwrap();
        
        println!("{}\n", random_quote);
        // println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
    };
}

