use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use colored::*;
use futures::stream::FuturesUnordered;
use std::time::Duration;
use std::{
    io::ErrorKind,
    net::{Shutdown, SocketAddr},
};

pub struct Scanner {
    host: String,
    start: u64,
    end: u64,
    batch_size: u64,
    timeout: Duration,
    quiet: bool,
}

impl Scanner {
    pub fn new(
        host: &str,
        start: u64,
        end: u64,
        batch_size: u64,
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

    pub async fn run(&self) -> Vec<u64> {
        let ports: Vec<u64> = (self.start..self.end).collect();
        let mut open_ports: std::vec::Vec<u64> = Vec::new();

        for range in ports.chunks(self.batch_size as usize) {
            let mut ports = self.scan_range(range).await;
            open_ports.append(&mut ports);
        }

        open_ports
    }

    async fn scan_range(&self, range: &[u64]) -> Vec<u64> {
        let mut ftrs = FuturesUnordered::new();

        for port in range {
            ftrs.push(self.scan_port(port));
        }

        let mut open_ports: Vec<u64> = Vec::new();
        while let Some(result) = ftrs.next().await {
            match result {
                Ok(port) => open_ports.push(port),
                _ => {}
            }
        }

        open_ports
    }

    async fn scan_port(&self, port: &u64) -> io::Result<u64> {
        let addr = format!("{}:{}", self.host, port);

        match addr.parse() {
            Ok(sock_addr) => match self.connect(sock_addr).await {
                Ok(stream_result) => {
                    match stream_result.shutdown(Shutdown::Both) {
                        _ => {}
                    }
                    if !self.quiet {
                        println!("Open {}", port.to_string().purple());
                    }

                    Ok(*port)
                }
                Err(e) => match e.kind() {
                    ErrorKind::Other => {
                        eprintln!("{:?}", e); // in case we get too many open files
                        panic!("Too many open files. Please reduce batch size. The default is 5000. Try -b 2500.");
                    }
                    _ => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
                },
            },
            Err(e) => {
                eprintln!("Unable to convert to socket address {:?}", e);
                panic!("Unable to convert to socket address");
            }
        }
    }

    async fn connect(&self, addr: SocketAddr) -> io::Result<TcpStream> {
        let stream =
            io::timeout(self.timeout, async move { TcpStream::connect(addr).await }).await?;
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn does_it_run() {
        // TODO run functions here
        assert_eq!(1, 1);
    }
}