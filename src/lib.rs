//! This crate exposes the internal functionality of the
//! [RustScan](https://rustscan.github.io/RustScan) port scanner.
//!
//! ## Example: perform a scan against localhost
//!
//! The core scanning behaviour is managed by
//! [`Scanner`](crate::scanner::Scanner) which in turn requires a
//! [`PortStrategy`](crate::port_strategy::PortStrategy):
//!
//! ```rust
//! use async_std::task::block_on;
//! use std::{net::IpAddr, time::Duration};
//!
//! use rustscan::input::{PortRange, ScanOrder};
//! use rustscan::port_strategy::PortStrategy;
//! use rustscan::scanner::Scanner;
//!
//! fn main() {
//!     let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
//!     let range = PortRange {
//!         start: 1,
//!         end: 1_000,
//!     };
//!     let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random); // can be serial, random or manual https://github.com/RustScan/RustScan/blob/master/src/port_strategy/mod.rs
//!     let scanner = Scanner::new(
//!         &addrs, // the addresses to scan
//!         10, // batch_size is how many ports at a time should be scanned
//!         Duration::from_millis(100), //T imeout is the time RustScan should wait before declaring a port closed. As datatype Duration.
//!         1, // Tries, how many retries should RustScan do?
//!         true, // greppable is whether or not RustScan should print things, or wait until the end to print only the ip
//!         strategy, // the port strategy used
//!         true, // accessible, should the output be A11Y compliant?
//!         vec![9000], // What ports should RustScan exclude?
//!         false, // is this a UDP scan?
//!     );
//!
//!     let scan_result = block_on(scanner.run());
//!
//!     println!("{:?}", scan_result);
//! }
//! ```
#![allow(clippy::needless_doctest_main)]

use std::sync::OnceLock;

pub mod tui;

pub mod input;

pub mod scanner;

pub mod port_strategy;

pub mod benchmark;

pub mod scripts;

pub mod address;

pub mod generated;

/// Static variable defining the current state of execution. The cli binary should
/// set it to true by calling set_cli_mode.
#[doc(hidden)]
pub static IS_CLI_MODE: OnceLock<bool> = OnceLock::new();

/// Set IS_CLI_MODE to true.
#[doc(hidden)]
pub fn set_cli_mode() {
    let _ = IS_CLI_MODE.set(true);
}
