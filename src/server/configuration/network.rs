use std::fmt;
use std::net::{IpAddr, Ipv4Addr, TcpListener};

#[derive(Debug)]
pub enum ServerPort {
    Empty,
    Is(u16),
}
impl Default for ServerPort {
    fn default() -> Self {
        ServerPort::Is(8080)
    }
}

impl fmt::Display for ServerPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerPort::Empty => write!(f, "empty"),
            ServerPort::Is(port) => write!(f, "{}", port),
        }
    }
}
#[derive(Debug)]
pub enum ServerListeningAddress {
    Empty,
    Is(IpAddr),
}
impl Default for ServerListeningAddress {
    fn default() -> Self {
        ServerListeningAddress::Is(IpAddr::V4(Ipv4Addr::LOCALHOST))
    }
}

impl fmt::Display for ServerListeningAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerListeningAddress::Empty => write!(f, "empty"),
            ServerListeningAddress::Is(addr) => write!(f, "{}", addr),
        }
    }
}

pub fn bind_listener(addr_str: &str, port: u16) -> Result<TcpListener, std::io::Error> {
    if let Ok(ipv6) = addr_str.parse::<std::net::Ipv6Addr>() {
        match TcpListener::bind((ipv6, port)) {
            Ok(l) => return Ok(l),
            Err(e) => tracing::warn!("IPv6 bind failed on [{}]: {} — trying IPv4", addr_str, e),
        }
    }

    if let Ok(ipv4) = addr_str.parse::<std::net::Ipv4Addr>() {
        return TcpListener::bind((ipv4, port));
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrNotAvailable,
        format!("Address '{}' is neither valid IPv4 nor IPv6", addr_str),
    ))
}
