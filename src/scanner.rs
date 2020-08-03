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
    host: IpAddr,
    start: u16,
    end: u16,
    batch_size: u32,
    timeout: Duration,
    quiet: bool,
}

impl Scanner {
    pub fn new(
        host: IpAddr,
        start: u16,
        end: u16,
        batch_size: u32,
        timeout: Duration,
        quiet: bool,
    ) -> Self {
        Self {
            host: host.to_owned(),
            start,
            end,
            batch_size,
            timeout,
            quiet,
        }
    }

    /// Runs scan_range with chunk sizes
    /// If you want to run RustScan normally, this is the entry point used
    /// Returns all open ports as Vec<u16>
    pub async fn run(&self) -> Vec<u16> {
        let ports: Vec<u16> = (self.start..self.end).collect();
        let mut open_ports: std::vec::Vec<u16> = Vec::new();
        // TODO change this to port size
        // to fix bug when we introduce custom port ranges

        for range in ports.chunks(self.batch_size as usize) {
            let mut ports = self.scan_range(range).await;
            open_ports.append(&mut ports);
        }

        open_ports
    }

    /// Given a range of ports, scan them all.
    /// Returns a vector of open ports.
    async fn scan_range(&self, range: &[u16]) -> Vec<u16> {
        let mut ftrs = FuturesUnordered::new();
        for port in range {
            ftrs.push(self.scan_port(*port));
        }

        let mut open_ports: Vec<u16> = Vec::new();
        while let Some(result) = ftrs.next().await {
            match result {
                Ok(port) => open_ports.push(port),
                _ => {}
            }
        }

        open_ports
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
    async fn scan_port(&self, port: u16) -> io::Result<u16> {
        let addr = SocketAddr::new(self.host, port);
        // println!("{:?}", addr);
        match self.connect(addr).await {
            Ok(x) => {
                // match stream_result.shutdown(Shutdown::Both)
                info!("Shutting down stream");
                match x.shutdown(Shutdown::Both) {
                    _ => {}
                }
                if !self.quiet {
                    println!("Open {}", port.to_string().purple());
                }
                // if connection successful
                // shut down stream
                // return port
                Ok(port)
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
    ///     let addr = SocketAddr::new(host, port)
    ///     self.connect(addr)
    ///     // returns Result which is either Ok(stream) for port is open, or Er for port is closed.
    ///     // Timeout occurs after self.timeout seconds
    ///
    async fn connect(&self, addr: SocketAddr) -> io::Result<TcpStream> {
        let stream =
            io::timeout(self.timeout, async move { TcpStream::connect(addr).await }).await?;
        info!("Returning okay from connect");
        Ok(stream)
    }
}
