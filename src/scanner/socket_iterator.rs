use std::net::{IpAddr, SocketAddr};

pub struct SocketIterator<'s> {
    ips: &'s [IpAddr],
    ports: &'s [u16],
    ip_idx: usize,
    ip_len: usize,
    port_idx: usize,
    port_len: usize,
}

/// An iterator that receives a slice of IPs and ports and returns a Socket
/// for each IP and port pair until all of these combinations are exhausted.
/// The goal of this iterator is to go over every IP and port combination
/// wihout generating a big memory footprint. The alternative would be
/// generating a vector containing all these combinations.
impl<'s> SocketIterator<'s> {
    pub fn new(ips: &'s [IpAddr], ports: &'s [u16]) -> Self {
        Self {
            ip_idx: 0,
            ip_len: ips.len(),
            port_idx: 0,
            port_len: ports.len(),
            ips,
            ports,
        }
    }
}

impl<'s> Iterator for SocketIterator<'s> {
    type Item = SocketAddr;

    /// Returns a socket based on the combination of one of the provided
    /// IPs and ports or None when these combinations are exhausted. Every
    /// IP will have the same port until a port is incremented.
    ///
    /// let it = SocketIterator::new(&["127.0.0.1", "192.168.0.1"], &[80, 443]);
    /// it.next(); // 127.0.0.1:80
    /// it.next(); // 192.168.0.1:80
    /// it.next(); // 127.0.0.1:443
    /// it.next(); // 192.168.0.1:443
    /// it.next(); // None
    fn next(&mut self) -> Option<Self::Item> {
        if self.port_idx == self.port_len {
            return None;
        }

        self.ip_idx = self.ip_idx % self.ip_len;

        let socket = SocketAddr::new(self.ips[self.ip_idx], self.ports[self.port_idx]);
        self.ip_idx += 1;

        if self.ip_idx == self.ip_len {
            self.port_idx += 1;
        }

        Some(socket)
    }
}

#[cfg(test)]
mod tests {
    use super::SocketIterator;
    use std::net::{IpAddr, SocketAddr};

    #[test]
    fn goes_through_every_ip_port_combination() {
        let addrs = vec![
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            "192.168.0.1".parse::<IpAddr>().unwrap(),
        ];
        let ports: Vec<u16> = vec![22, 80, 443];
        let mut it = SocketIterator::new(&addrs, &ports);

        assert_eq!(Some(SocketAddr::new(addrs[0], ports[0])), it.next());
        assert_eq!(Some(SocketAddr::new(addrs[1], ports[0])), it.next());
        assert_eq!(Some(SocketAddr::new(addrs[0], ports[1])), it.next());
        assert_eq!(Some(SocketAddr::new(addrs[1], ports[1])), it.next());
        assert_eq!(Some(SocketAddr::new(addrs[0], ports[2])), it.next());
        assert_eq!(Some(SocketAddr::new(addrs[1], ports[2])), it.next());
        assert_eq!(None, it.next());
    }
}
