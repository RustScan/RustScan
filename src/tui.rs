//! Utilities for terminal output during scanning.

/// Terminal User Interface Module for RustScan
/// Defines macros to use
#[macro_export]
macro_rules! warning {
    ($name:expr) => {
        $crate::print_log!(
            warn,
            "{} {}",
            ansi_term::Colour::Red.bold().paint("[!]"),
            $name
        );
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                $crate::print_log!(warn, "{}", $name);
            } else {
                $crate::print_log!(
                    warn,
                    "{} {}",
                    ansi_term::Colour::Red.bold().paint("[!]"),
                    $name
                );
            }
        }
    };
}

#[macro_export]
macro_rules! detail {
    ($name:expr) => {
        $crate::print_log!(
            info,
            "{} {}",
            ansi_term::Colour::Blue.bold().paint("[~]"),
            $name
        );
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                $crate::print_log!(info, "{}", $name);
            } else {
                $crate::print_log!(
                    info,
                    "{} {}",
                    ansi_term::Colour::Blue.bold().paint("[~]"),
                    $name
                );
            }
        }
    };
}

#[macro_export]
macro_rules! output {
    ($name:expr) => {
        $crate::print_log!(
            info,
            "{} {}",
            RGansi_term::Colour::RGB(0, 255, 9).bold().paint("[>]"),
            $name
        );
    };
    ($name:expr, $greppable:expr, $accessible:expr) => {
        // if not greppable then print, otherwise no else statement so do not print.
        if !$greppable {
            if $accessible {
                // Don't print the ascii art
                $crate::print_log!(info, "{}", $name);
            } else {
                $crate::print_log!(
                    info,
                    "{} {}",
                    ansi_term::Colour::RGB(0, 255, 9).bold().paint("[>]"),
                    $name
                );
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
            "I don't always scan ports, but when I do, I prefer RustScan.",
            "RustScan: Where scanning meets swagging. 😎",
            "To scan or not to scan? That is the question.",
            "RustScan: Because guessing isn't hacking.",
            "Scanning ports like it's my full-time job. Wait, it is.",
            "Open ports, closed hearts.",
            "I scanned my computer so many times, it thinks we're dating.",
            "Port scanning: Making networking exciting since... whenever.",
            "You miss 100% of the ports you don't scan. - RustScan",
            "Breaking and entering... into the world of open ports.",
            "TCP handshake? More like a friendly high-five!",
            "Scanning ports: The virtual equivalent of knocking on doors.",
            "RustScan: Making sure 'closed' isn't just a state of mind.",
            "RustScan: allowing you to send UDP packets into the void 1200x faster than NMAP",
            "Port scanning: Because every port has a story to tell.",
            "I scanned ports so fast, even my computer was surprised.",
            "Scanning ports faster than you can say 'SYN ACK'",
            "RustScan: Where '404 Not Found' meets '200 OK'.",
            "RustScan: Exploring the digital landscape, one IP at a time.",
            "TreadStone was here 🚀",
            "With RustScan, I scan ports so fast, even my firewall gets whiplash 💨",
        ];
        let random_quote = quotes.choose(&mut rand::thread_rng()).unwrap();

        println!("{}\n", random_quote);
    };
}

/// Wrapper macro for printing/logging wraps println! and log::$level!
/// 1. if rustscan::IS_CLI_MODE is true calls `println!`
/// 2. if rustscan::IS_CLI_MODE is undefined or false `log::$level!` also sets IS_CLI_MODE
/// to false if it was previously undefined.
///
/// Library code should call this macro to print information that the binary
/// is expected to print to stdout and library is expected to log at a
/// level specified by parameter $level.
#[doc(hidden)]
#[macro_export]
macro_rules! print_log {
    ($level:ident, $($fmt_args:tt)*) => {
        if *$crate::IS_CLI_MODE.get_or_init(|| false) {
            println!($($fmt_args)*);
        } else {
            log::$level!($($fmt_args)*);
        }
    }
}
