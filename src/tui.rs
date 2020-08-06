/// Terminal User Interface Module for RustScan
/// Defines macros to use

// TODO make them work with formatted strings

#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        use ansi_term::Colour::Red;
        println!("{} {}", Red.paint("[!]"), $name);
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        use ansi_term::Colour::Blue;
        println!("{} {}", Blue.paint("[!]"), $name);
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr) => {
        use ansi_term::Colour::Green;
        println!("{} {}", Green.paint("[!]"), $name);
    };
}