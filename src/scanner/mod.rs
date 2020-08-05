mod range_iterator;
use range_iterator::RangeIterator;

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use colored::*;
use futures::stream::FuturesUnordered;
use std::time::Duration;
use std::{
    io::ErrorKind,
    net::{IpAddr, Shutdown, SocketAddr},
};

/// The class for the scanner
/// Host is data type IpAddr and is the host address
/// start & end is where the port scan starts and ends
/// batch_size is how many ports at a time should be scanned
/// Timeout is the time RustScan should wait before declaring a port closed. As datatype Duration.
/// Quiet is whether or not RustScan should print things, or wait until the end to print only open ports.
#[cfg(not(tarpaulin_include))]
pub struct Scanner {
    hosts: Vec<IpAddr>,
    start: u16,
    end: u16,
    batch_size: u16,
    timeout: Duration,
    quiet: bool,
}

impl Scanner {
    pub fn new(
        hosts: &[IpAddr],
        start: u16,
        end: u16,
        batch_size: u16,
        timeout: Duration,
        quiet: bool,
    ) -> Self {
        Self {
            start,
            end,
            batch_size,
            timeout,
            quiet,
            hosts: hosts.iter().map(|host| host.to_owned()).collect(),
        }
    }

    /// Runs scan_range with chunk sizes
    /// If you want to run RustScan normally, this is the entry point used
    /// Returns all open ports as Vec<u16>
    pub async fn run(&self) -> Vec<SocketAddr> {
        let range_iterator = RangeIterator::new(self.start.into(), self.end.into());
        let mut sockets: Vec<SocketAddr> = Vec::new();
        let mut open_sockets: Vec<SocketAddr> = Vec::new();

        for port in range_iterator {
            for host in &self.hosts {
                sockets.push(SocketAddr::new(*host, port));
            }

            if sockets.len() >= self.batch_size.into() {
                let mut results = self.scan_range(&sockets).await;
                open_sockets.append(&mut results);
                sockets = Vec::new();
            }
        }

        // This will happen if we have a number of sockets remaining
        // that is lower than the batch size.
        if !sockets.is_empty() {
            let mut results = self.scan_range(&sockets).await;
            open_sockets.append(&mut results);
        }

        open_sockets
    }

    /// Given a slice of sockets, scan them all.
    /// Returns a vector of open sockets.
    async fn scan_range(&self, sockets: &[SocketAddr]) -> Vec<SocketAddr> {
        let mut ftrs = FuturesUnordered::new();
        for socket in sockets {
            ftrs.push(self.scan_socket(socket));
        }

        let mut open_sockets: Vec<SocketAddr> = Vec::new();
        while let Some(result) = ftrs.next().await {
            match result {
                Ok(socket) => open_sockets.push(socket),
                _ => {}
            }
        }

        open_sockets
    }

    /// Given a port, scan it.
    /// Turns the address into a SocketAddr
    /// Deals with the <result> type
    /// If it experiences error ErrorKind::Other then too many files are open and it Panics!
    /// ese any other error, it returns the error in Result as a string
    /// If no  errors occur, it returns the port number in Result to signify the port is open.
    /// This function mainly deals with the logic of Results handling.
    /// # Example
    ///
    ///     self.scan_port(10:u16)
    ///
    /// Note: `self` must contain `self.host`.
    async fn scan_socket(&self, socket: &SocketAddr) -> io::Result<SocketAddr> {
        match self.connect(*socket).await {
            Ok(x) => {
                // match stream_result.shutdown(Shutdown::Both)
                info!("Shutting down stream");
                match x.shutdown(Shutdown::Both) {
                    _ => {}
                }
                if !self.quiet {
                    println!("Open {}", socket.to_string().purple());
                }

                Ok(*socket)
            }
            Err(e) => match e.kind() {
                ErrorKind::Other => {
                    panic!("Too many open files. Please reduce batch size. The default is 5000. Try -b 2500.");
                }
                _ => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        }
    }

    /// Performs the connection to the socket with timeout
    /// # Example
    ///
    ///     let port: u16 = 80
    ///     // Host is an IpAddr type
    ///     let host = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    ///     let socket = SocketAddr::new(host, port);
    ///     self.connect(socket)
    ///     // returns Result which is either Ok(stream) for port is open, or Er for port is closed.
    ///     // Timeout occurs after self.timeout seconds
    ///
    async fn connect(&self, socket: SocketAddr) -> io::Result<TcpStream> {
        let stream = io::timeout(
            self.timeout,
            async move { TcpStream::connect(socket).await },
        )
        .await?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task::block_on;
    use std::{net::IpAddr, time::Duration};

    #[test]
    fn scanner_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
        let scanner = Scanner::new(&addrs, 1, 1_000, 10, Duration::from_millis(100), true);
        block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[test]
    fn ipv6_scanner_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = vec!["::1".parse::<IpAddr>().unwrap()];
        let scanner = Scanner::new(&addrs, 1, 1_000, 10, Duration::from_millis(100), false);
        block_on(scanner.run());
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[test]
    fn quad_zero_scanner_runs() {
        let addrs = vec!["0.0.0.0".parse::<IpAddr>().unwrap()];
        let scanner = Scanner::new(&addrs, 1, 1_000, 10, Duration::from_millis(500), true);
        block_on(scanner.run());
        assert_eq!(1, 1);
    }
    #[test]
    fn google_dns_runs() {
        let addrs = vec!["8.8.8.8".parse::<IpAddr>().unwrap()];
        let scanner = Scanner::new(&addrs, 400, 445, 10, Duration::from_millis(1_500), true);
        block_on(scanner.run());
        assert_eq!(1, 1);
    }
    #[test]
    fn infer_ulimit_lowering_no_panic() {
        // Test behaviour on MacOS where ulimit is not automatically lowered
        let addrs = vec!["8.8.8.8".parse::<IpAddr>().unwrap()];

        // mac should have this automatically scaled down
        let scanner = Scanner::new(&addrs, 400, 600, 10_000, Duration::from_millis(1500), true);
        block_on(scanner.run());
        assert_eq!(1, 1);
    }
}
