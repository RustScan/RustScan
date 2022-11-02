/// Terminal User Interface Module for RustScan
/// Defines macros to use

#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        use ansi_term::Colour::Red;
        println!("{} {}", Red.bold().paint("[!]"), $name);
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::Red;
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!("{} {}", Red.bold().paint("[!]"), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        use ansi_term::Colour::Blue;
        println!("{} {}", Blue.bold().paint("[~]"), $name);
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::Blue;
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!("{} {}", Blue.bold().paint("[~]"), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr) => {
        use ansi_term::Colour::RGB;
        println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::RGB;
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                println!("{}", $name);
            } else {
                println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
            }
        }
    };
}

#[macro_export]
macro_rules! funny_opening {
    // prints a funny quote / opening
    () => {
        use rand::seq::SliceRandom;
        let quotes = vec![
            "Nmap? More like slowmap.🐢",
            "🌍HACK THE PLANET🌍",
            "Real hackers hack time ⌛",
            "Please contribute more quotes to our GitHub https://github.com/rustscan/rustscan",
            "😵 https://admin.tryhackme.com",
            "0day was here ♥",
        ];
        let random_quote = quotes.choose(&mut rand::thread_rng()).unwrap();

        println!("{}\n", random_quote);
        // println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
    };
}
