//! Core functionality for actual scanning behaviour.
use crate::generated::get_parsed_data;
use crate::port_strategy::PortStrategy;
use log::debug;

mod socket_iterator;
use socket_iterator::SocketIterator;

use colored::Colorize;
use std::collections::BTreeMap;
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
    num::NonZero,
    time::Duration,
};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use futures::StreamExt;
use tokio::{io, time};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, UdpSocket};
use tokio_par_stream::FuturesParallelUnordered;

#[derive(Debug)]
pub struct ScannerInner {
    ips: Box<[IpAddr]>,
    exclude_ports: Box<[u16]>,
    
    batch_size: u16,
    timeout: Duration,
    tries: NonZero<u8>,
    greppable: bool,
    port_strategy: PortStrategy,
    accessible: bool,
    udp: bool,
}

/// The class for the scanner
/// IP is data type IpAddr and is the IP address
/// start & end is where the port scan starts and ends
/// batch_size is how many ports at a time should be scanned
/// Timeout is the time RustScan should wait before declaring a port closed. As datatype Duration.
/// greppable is whether RustScan should print things, or wait until the end to print only the ip and open ports.
#[cfg(not(tarpaulin_include))]
#[derive(Debug)]
pub struct Scanner(Arc<ScannerInner>);

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
        exclude_ports: &[u16],
        udp: bool,
    ) -> Self {
        let inner = ScannerInner {
            ips: Box::from(ips),
            exclude_ports: Box::from(exclude_ports),
            
            batch_size,
            timeout,
            tries: NonZero::new(tries).unwrap_or(NonZero::<u8>::MIN),
            greppable,
            port_strategy,
            
            accessible,
            udp,
        };
        
        Self(Arc::new(inner))
    }
    
    pub async fn run(&self) -> Vec<SocketAddr> {
        self.0.run().await
    }
}

impl ScannerInner {
        /// Runs scan_range with chunk sizes
    /// If you want to run RustScan normally, this is the entry point used
    /// Returns all open ports as `Vec<u16>`
    async fn run(self: &Arc<Self>) -> Vec<SocketAddr> {
        let ports = self
            .port_strategy
            .iter()
            .filter(|port| !self.exclude_ports.contains(port))
            .collect::<Box<[u16]>>();
        
        let mut socket_iterator: SocketIterator = SocketIterator::new(&self.ips, &ports);
        let mut open_sockets: Vec<SocketAddr> = Vec::new();
        let mut ftrs = FuturesParallelUnordered::new();
        let mut errors: HashSet<String> = HashSet::new();
        let udp_map = get_parsed_data();
        let scan = |socket| {
            let this = Arc::clone(self);
            async move { this.scan_socket(socket, udp_map).await }
        };
            
        for _ in 0..self.batch_size {
            if let Some(socket) = socket_iterator.next() {
                ftrs.push(scan(socket));
            } else {
                break;
            }
        }

        debug!("Start scanning sockets. \nBatch size {}\nNumber of ip-s {}\nNumber of ports {}\nTargets all together {} ",
            self.batch_size,
            self.ips.len(),
            &ports.len(),
            self.ips.len() * ports.len());

        while let Some(result) = ftrs.next().await {
            if let Some(socket) = socket_iterator.next() {
                ftrs.push(scan(socket));
            }

            match result {
                Ok(socket) => open_sockets.push(socket),
                Err(e) => {
                    let error_string = e.to_string();
                    if errors.len() < self.ips.len() * 1000 {
                        errors.insert(error_string);
                    }
                }
            }
        }
        debug!("Typical socket connection errors {:?}", errors);
        debug!("Open Sockets found: {:?}", &open_sockets);
        open_sockets
    }

    /// Given a socket, scan itself. Tries times.
    /// Turns the address into a SocketAddr
    /// Deals with the `<result>` type
    /// If it experiences error `ErrorKind::Other` then too many files are open and it Panics!
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
    async fn scan_socket(
        &self,
        socket: SocketAddr,
        udp_map: &'static BTreeMap<Vec<u16>, Vec<u8>>,
    ) -> io::Result<SocketAddr> {
        if self.udp {
            return self.scan_udp_socket(socket, udp_map).await;
        }

        let tries = self.tries.get();
        for nr_try in 1..=tries {
            match self.connect(socket).await {
                Ok(mut tcp_stream) => {
                    debug!(
                        "Connection was successful, shutting down stream {}",
                        &socket
                    );
                    if let Err(e) = tcp_stream.shutdown().await {
                        debug!("Shutdown stream error {}", &e);
                    }
                    self.fmt_ports(socket);

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

    async fn scan_udp_socket(
        &self,
        socket: SocketAddr,
        udp_map: &'static BTreeMap<Vec<u16>, Vec<u8>>,
    ) -> io::Result<SocketAddr> {
        let mut payload = &[][..];
        for (key, value) in udp_map {
            if key.contains(&socket.port()) {
                payload = value;
            }
        }

        let tries = self.tries.get();
        for _ in 1..=tries {
            match self.udp_scan(socket, payload, self.timeout).await {
                Ok(true) => return Ok(socket),
                Ok(false) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(socket)
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
        time::timeout(
            self.timeout,
            async move { TcpStream::connect(socket).await },
        )
        .await?
    }

    /// Binds to a UDP socket so we can send and receive packets
    /// # Example
    ///
    /// ```compile_fail
    /// # use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    /// let port: u16 = 80;
    /// // ip is an IpAddr type
    /// let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// let socket = SocketAddr::new(ip, port);
    /// scanner.udp_bind(socket);
    /// // returns Result which is either Ok(stream) for port is open, or Err for port is closed.
    /// // Timeout occurs after self.timeout seconds
    /// ```
    ///
    async fn udp_bind(&self, socket: SocketAddr) -> io::Result<UdpSocket> {
        let local_addr = match socket {
            SocketAddr::V4(_) => SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)),
            SocketAddr::V6(_) => SocketAddr::from((Ipv6Addr::UNSPECIFIED, 0)),
        };

        UdpSocket::bind(local_addr).await
    }

    /// Performs a UDP scan on the specified socket with a payload and wait duration
    /// # Example
    ///
    /// ```compile_fail
    /// # use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    /// # use std::time::Duration;
    /// let port: u16 = 123;
    /// // ip is an IpAddr type
    /// let ip = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// let socket = SocketAddr::new(ip, port);
    /// let payload = vec![0, 1, 2, 3];
    /// let wait = Duration::from_secs(1);
    /// let result = scanner.udp_scan(socket, payload, wait).await;
    /// // returns Result which is either Ok(true) if response received, or Ok(false) if timed out.
    /// // Err is returned for other I/O errors.
    async fn udp_scan(
        &self,
        socket: SocketAddr,
        payload: &[u8],
        wait: Duration,
    ) -> io::Result<bool> {
        match self.udp_bind(socket).await {
            Ok(udp_socket) => {
                let mut buf = [0u8; 1024];

                udp_socket.connect(socket).await?;
                udp_socket.send(payload).await?;

                match time::timeout(wait, udp_socket.recv(&mut buf)).await {
                    Ok(Ok(size)) => {
                        debug!("Received {} bytes", size);
                        self.fmt_ports(socket);
                        Ok(true)
                    }
                    // timeout
                    Err(_) => Ok(false),
                    Ok(Err(e)) => Err(e),
                }
            }
            Err(e) => {
                println!("Err E binding sock {:?}", e);
                Err(e)
            }
        }
    }

    /// Formats and prints the port status
    fn fmt_ports(&self, socket: SocketAddr) {
        if !self.greppable {
            if self.accessible {
                println!("Open {socket}");
            } else {
                println!("Open {}", socket.to_string().purple());
            }
        }
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
        let addrs = ["127.0.0.1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            false,
        );
        scanner.run().await;
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn ipv6_scanner_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = ["::1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            false,
        );
        scanner.run().await;
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn quad_zero_scanner_runs() {
        let addrs = ["0.0.0.0".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            false,
        );
        scanner.run().await;
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn google_dns_runs() {
        let addrs = ["8.8.8.8".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 400,
            end: 445,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            false,
        );
        scanner.run().await;
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn infer_ulimit_lowering_no_panic() {
        // Test behaviour on macOS where ulimit is not automatically lowered
        let addrs = ["8.8.8.8".parse::<IpAddr>().unwrap()];

        // mac should have this automatically scaled down
        let range = PortRange {
            start: 400,
            end: 600,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            false,
        );
        scanner.run().await;
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn udp_scan_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = ["127.0.0.1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            true,
        );
        scanner.run().await;
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn udp_ipv6_runs() {
        // Makes sure the program still runs and doesn't panic
        let addrs = ["::1".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            true,
        );
        scanner.run().await;
        // if the scan fails, it wouldn't be able to assert_eq! as it panicked!
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn udp_quad_zero_scanner_runs() {
        let addrs = ["0.0.0.0".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 1,
            end: 1_000,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            true,
        );
        scanner.run().await;
        assert_eq!(1, 1);
    }
    #[tokio::test]
    async fn udp_google_dns_runs() {
        let addrs = ["8.8.8.8".parse::<IpAddr>().unwrap()];
        let range = PortRange {
            start: 100,
            end: 150,
        };
        let strategy = PortStrategy::range(range, ScanOrder::Random);
        let scanner = Scanner::new(
            &addrs,
            10,
            Duration::from_millis(100),
            1,
            true,
            strategy,
            true,
            &[9000],
            true,
        );
        scanner.run().await;
        assert_eq!(1, 1);
    }
}
