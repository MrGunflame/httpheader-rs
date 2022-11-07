use std::str::FromStr;

use crate::error::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ipv4Addr(std::net::Ipv4Addr);

impl FromStr for Ipv4Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(addr) => Ok(Self(addr)),
            Err(_) => Err(Error {
                position: 0,
                expected: "IPv4 address",
            }),
        }
    }
}

impl PartialEq<std::net::Ipv4Addr> for Ipv4Addr {
    fn eq(&self, other: &std::net::Ipv4Addr) -> bool {
        self.0 == *other
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ipv6Addr(std::net::Ipv6Addr);

impl FromStr for Ipv6Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(addr) => Ok(Self(addr)),
            Err(_) => Err(Error {
                position: 0,
                expected: "IPv6 address",
            }),
        }
    }
}

impl PartialEq<std::net::Ipv6Addr> for Ipv6Addr {
    fn eq(&self, other: &std::net::Ipv6Addr) -> bool {
        self.0 == *other
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Port(pub u16);

impl FromStr for Port {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(port) => Ok(Self(port)),
            Err(_) => Err(Error {
                position: 0,
                expected: "port",
            }),
        }
    }
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Self(port)
    }
}
