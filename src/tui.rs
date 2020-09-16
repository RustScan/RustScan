/// Terminal User Interface Module for RustScan
/// Defines macros to use

#[macro_export]
macro_rules! warning {
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::Red;
        // if greppable, no else statement so do not print.
        if !$greppable {
            println!("{} {}", Red.bold().paint("[!]"), $name);
        }
        else if !$accessible{
            // Don't print the ascii art
            println!("{}", $name);
        }
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::Blue;
        // if greppable, no else statement so do not print.
        if !$greppable {
            println!("{} {}", Blue.bold().paint("[~]"), $name);
        }
        else if !$accessible{
            // Don't print the ascii art
            println!("{}", $name);
        }
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr, $greppable:expr, $accessible:expr) => {
        use ansi_term::Colour::RGB;
        // if greppable, no else statement so do not print.
        if !$greppable {
            println!("{} {}", RGB(0, 255, 9).bold().paint("[>]"), $name);
        }
        else if !$accessible{
            // Don't print the ascii art
            println!("{}", $name);
        }
    };
}

#[macro_export]
macro_rules! funny_opening {
    // prints a funny quote / opening
    () => {
        use rand::seq::SliceRandom;
        let quotes = vec![
            "Hello, Elliot.",
            "🌍HACK THE PLANET🌍",
            "Real hackers hack time ⌛",
            "Please contribute more quotes to our GitHub https://github.com/rustscan/rustscan",
            "😵 https://admin.tryhackme.com",
            "Mess with the best, die like the rest",
        ];
        let random_quote = quotes.choose(&mut rand::thread_rng()).unwrap();

        println!("{}\n", random_quote);
    };
}
