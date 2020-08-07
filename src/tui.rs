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
