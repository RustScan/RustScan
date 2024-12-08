//! File used just to build a binary for testing if stdout is being logged.
//! Older versions of the library rustscan would write to stdout. This file
//! helps ensure the library only writes to stdout if log is initialized,
//! otherwise it shouldn't write at all.
//!
//! It was necessary to create this file because checking if some code write to
//! stdout is very orthogonal to rust's testing tools. There are utilities but
//! only on unstable rust. This file is used to create a binary that can
//! be executed as child process for testing the behavior.

#![allow(unused)]

use std::{net::IpAddr, str::FromStr, time::Duration};

use futures::executor::block_on;
use rustscan::{input::ScanOrder, port_strategy::PortStrategy, scanner::Scanner};

fn main() {
    // "open" tcp connection on random port
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    // get the port from above connection
    let port = listener.local_addr().unwrap().port();

    // execute
    block_on(
        Scanner::new(
            &[IpAddr::from_str("127.0.0.1").unwrap()],
            100,
            Duration::from_secs(5),
            3,
            false,
            PortStrategy::pick(&None, Some(vec![port]), ScanOrder::Random),
            true,
            vec![],
            false,
        )
        .run(),
    );
}
