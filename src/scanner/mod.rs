//! Core functionality for actual scanning behaviour.
use crate::port_strategy::PortStrategy;
use log::debug;

mod socket_iterator;
use socket_iterator::SocketIterator;

use tokio::io;
use tokio::net::TcpStream;
use colored::Colorize;
use std::{
    net::{IpAddr, SocketAddr},
    num::NonZeroU8,
    time::Duration,
};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;

/// The class for the scanner
/// IP is data type IpAddr and is the IP address
/// start & end is where the port scan starts and ends
/// batch_size is how many ports at a time should be scanned
/// Timeout is the time RustScan should wait before declaring a port closed. As datatype Duration.
/// greppable is whether or not RustScan should print things, or wait until the end to print only the ip and open ports.
/// Added by wasuaje - 01/26/2024:
///     exclude_ports  is an exclusion port list
#[cfg(not(tarpaulin_include))]
#[derive(Debug)]
struct ScannerInner {
    ips: Box<[IpAddr]>,
    batch_size: u16,
    timeout: Duration,
    tries: NonZeroU8,
    greppable: bool,
    port_strategy: PortStrategy,
    accessible: bool,
    exclude_ports: Box<[u16]>,
}

#[derive(Clone)]
pub struct Scanner(Arc<ScannerInner>);
impl Debug for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <ScannerInner as Debug>::fmt(&*self.0, f)
    }
}

// Allowing too many arguments for clippy.
#[allow(clippy::too_many_arguments)]
impl Scanner {
    pub fn new(
        ips: &[IpAddr],
        batch_size: u16,
        timeout: Duration,
        tries: u8,
        greppable: bool,
        port_strategy: PortStrategy,
        accessible: bool,
        exclude_ports: Vec<u16>,
    ) -> Self {
        let inner = ScannerInner {
            batch_size,
            timeout,
            tries: NonZeroU8::new(tries).unwrap_or(NonZeroU8::MIN),
            greppable,
            port_strategy,
            ips: Box::from(ips),
            accessible,
            exclude_ports: exclude_ports.into_boxed_slice(),
        };
        
        Self(Arc::new(inner))
    }

    /// Runs scan_range with chunk sizes
    /// If you want to run RustScan normally, this is the entry point used
    /// Returns all open ports as `Vec<u16>`
    /// Added by wasuaje - 01/26/2024:
    ///    Filtering port against exclude port list
    pub async fn run(&self) -> Vec<SocketAddr> {
        let ports = {
            let mut ports = self.0
                .port_strategy
                .order();
            ports.retain(|port| !self.0.exclude_ports.contains(port));
            ports
        };

        let mut socket_iterator = SocketIterator::new(&self.0.ips, &ports);
        let mut open_sockets: Vec<SocketAddr> = Vec::new();
        let mut ftrs = JoinSet::new();
        let mut errors: HashSet<String> = HashSet::new();

        for _ in 0..self.0.batch_size {
            if let Some(socket) = socket_iterator.next() {
                ftrs.spawn(self.clone().scan_socket(socket));
            } else {
                break;
            }
        }

        debug!("Start scanning sockets. \nBatch size {}\nNumber of ip-s {}\nNumber of ports {}\nTargets all together {} ",
            self.0.batch_size,
            self.0.ips.len(),
            &ports.len(),
            self.0.ips.len() * ports.len());

        while let Some(result) = ftrs.join_next().await {
            if let Some(socket) = socket_iterator.next() {
                ftrs.spawn(self.clone().scan_socket(socket));
            }

            match result.map_err(io::Error::from) {
                Ok(Ok(socket)) => open_sockets.push(socket),
                Err(err) | Ok(Err(err)) => {
                    let error_string = err.to_string();
                    if errors.len() < self.0.ips.len() * 1000 {
                        errors.insert(error_string);
                    }
                }
            }
        }
        debug!("Typical socket connection errors {:?}", errors);
        debug!("Open Sockets found: {:?}", &open_sockets);
        open_sockets
    }

    /// Given a socket, scan it self.tries times.
    /// Turns the address into a SocketAddr
    /// Deals with the `<result>` type
    /// If it experiences error ErrorKind::Other then too many files are open and it Panics!
    /// Else any other error, it returns the error in Result as a string
    /// If no errors occur, it returns the port number in Result to signify the port is open.
    /// This function mainly deals with the logic of Results handling.
    /// # Example
    ///
    /// ```compile_fail
    /// scanner.scan_socket(socket)
    /// ```
    ///
    /// Note: `self` must contain `self.ip`.
    async fn scan_socket(self, socket: SocketAddr) -> io::Result<SocketAddr> {
        let tries = self.0.tries.get();

        for nr_try in 1..=tries {
            match self.connect(socket).await {
                Ok(mut x) => {
                    debug!(
                        "Connection was successful, shutting down stream {}",
                        &socket
                    );
                    if let Err(e) = x.shutdown().await {
                        debug!("Shutdown stream error {}", &e);
                    }
                    if !self.0.greppable {
                        if self.0.accessible {
                            println!("Open {socket}");
                        } else {
                            println!("Open {}", socket.to_string().purple());
                        }
                    }

                    debug!("Return Ok after {} tries", nr_try);
                    return Ok(socket);
                }
                Err(e) => {
                    let mut error_string = e.to_string();

                    assert!(!error_string.to_lowercase().contains("too many open files"), "Too many open files. Please reduce batch size. The default is 5000. Try -b 2500.");

                    if nr_try == tries {
                        error_string.push(' ');
                        error_string.push_str(&socket.ip().to_string());
                        return Err(io::Error::new(io::ErrorKind::Other, error_string));
                    }
                }
            };
        }
        unreachable!();
    }

    /// Performs the connection to the socket with timeout
    /// # Example
    ///
    /// ```compile_fail
    /// # use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    /// let port: u16 = 80;
    /// // ip is an IpAddr type
    /// let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// let socket = SocketAddr::new(ip, port);
    /// scanner.connect(socket);
    /// // returns Result which is either Ok(stream) for port is open, or Er for port is closed.
    /// // Timeout occurs after self.timeout seconds
    /// ```
    ///
    async fn connect(&self, socket: SocketAddr) -> io::Result<TcpStream> {
        let stream = tokio::time::timeout(
            self.0.timeout,
            async move { TcpStream::connect(socket).await },
        )
        .await??;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{PortRange, ScanOrder};
    use std::{net::IpAddr, time::Duration};

    #[tokio::test]
    async fn scanner_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            vec![9000],
        );
        scanner.run().await;
    }
    #[tokio::test]
    async fn ipv6_scanner_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = vec!["::1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            vec![9000],
        );

        scanner.run().await;
    }
    #[tokio::test]
    async fn quad_zero_scanner_runs() {
        let addrs = vec!["0.0.0.0".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            vec![9000],
        );
        scanner.run().await;
    }
    #[tokio::test]
    async fn google_dns_runs() {
        let addrs = vec!["8.8.8.8".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 400,
            end: 445,
        };
        let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            vec![9000],
        );
        scanner.run().await;
    }
    #[tokio::test]
    async fn infer_ulimit_lowering_no_panic() {
        // Test behaviour on MacOS where ulimit is not automatically lowered
        let addrs = vec!["8.8.8.8".parse::<IpAddr>().unwrap()];

        // mac should have this automatically scaled down
        let range = PortRange {
            start: 400,
            end: 600,
        };
        let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            vec![9000],
        );
        scanner.run().await;
    }
}
