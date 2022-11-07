use std::str::FromStr;

use crate::error::Error;
use crate::parser::Span;
use crate::types::{Ipv4Addr, Ipv6Addr, Port};

/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Forwarded
#[derive(Clone, Debug)]
pub struct Forwarded {
    /// The interface where the request came in to the proxy server.
    pub by: Option<Identifier>,
    /// The client that initiated the request and subsequent proxies in a chain of proxies.
    pub r#for: Vec<Identifier>,
    /// The Host request header field as received by the proxy.
    pub host: Option<String>,
    /// Indicates which protocol was used to make the request (typically "http" or "https").
    pub proto: Option<String>,
}

impl FromStr for Forwarded {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut by = None;
        let mut r#for = Vec::new();
        let mut host = None;
        let mut proto = None;

        for pair in s.split(';') {
            let (key, val) = match pair.split_once('=') {
                Some((key, val)) => (key, val),
                None => {
                    return Err(Error {
                        position: 0,
                        expected: "a key=value pair",
                    })
                }
            };

            match key {
                "by" => by = Some(val.parse()?),
                "for" => {
                    for (i, mut val) in val.split(',').enumerate() {
                        // Strip leading spaces.
                        val = val.trim_start();

                        if i != 0 {
                            val = match val.strip_prefix("for=") {
                                Some(val) => val,
                                None => {
                                    return Err(Error {
                                        position: 0,
                                        expected: "another `for=` value, or a semicolon",
                                    })
                                }
                            };
                        }

                        r#for.push(val.parse()?)
                    }
                }
                "host" => host = Some(val.to_owned()),
                "proto" => proto = Some(val.to_owned()),
                _ => {
                    return Err(Error {
                        position: 0,
                        expected: "one of by, for, host, proto",
                    })
                }
            }
        }

        Ok(Self {
            by,
            r#for,
            host,
            proto,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Identifier {
    Obfuscated(String),
    Ipv4(Ipv4Addr, Option<Port>),
    Ipv6(Ipv6Addr, Option<Port>),
    Unknown,
}

impl FromStr for Identifier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let span = Span::new(s);

        match span.enclosed('\"') {
            Ok(span) => {
                let s = span.as_str();

                // IPv6 address with optional port
                if s.len() > 2 && s.starts_with('[') && s.contains(']') {
                    let end = s.find(']').unwrap();
                    let ip = &s[1..end];

                    let addr = ip.parse()?;

                    let port = if end == s.len() - 1 {
                        None
                    } else {
                        let port = &s[end + 2..];
                        Some(port.parse()?)
                    };

                    return Ok(Self::Ipv6(addr, port));
                }

                match s {
                    "unknown" => Ok(Self::Unknown),
                    s => Ok(Self::Obfuscated(s.to_owned())),
                }
            }
            Err(_) => {
                // Ipv4 address with optional port
                let (addr, port) = span.split_once(':');

                let addr = addr.parse()?;
                let port = match port {
                    Some(port) => Some(port.parse()?),
                    None => None,
                };

                Ok(Self::Ipv4(addr, port))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Forwarded, Identifier};
    use crate::types::Port;

    #[test]
    fn forwarded_from_str() {
        let input = "by=127.0.0.1:80;for=127.0.0.2:81, for=\"[::]:82\", for=\"unknown\";host=localhost;proto=http";
        let header = Forwarded::from_str(input).unwrap();
        assert_eq!(
            header.by,
            Some(Identifier::Ipv4(
                "127.0.0.1".parse().unwrap(),
                Some(80.into())
            ))
        );
        assert_eq!(
            header.r#for,
            vec![
                Identifier::Ipv4("127.0.0.2".parse().unwrap(), Some(81.into())),
                Identifier::Ipv6("::".parse().unwrap(), Some(82.into())),
                Identifier::Unknown,
            ]
        );
        assert_eq!(header.host, Some("localhost".into()));
        assert_eq!(header.proto, Some("http".into()));
    }

    #[test]
    fn identifier_from_str() {
        let input = "\"_mdn\"";
        assert_eq!(
            Identifier::from_str(input).unwrap(),
            Identifier::Obfuscated("_mdn".into())
        );

        let input = "\"[2001:db8:cafe::17]\"";
        assert_eq!(
            Identifier::from_str(input).unwrap(),
            Identifier::Ipv6("2001:db8:cafe::17".parse().unwrap(), None)
        );

        let input = "\"[2001:db8:cafe::17]:4711\"";
        assert_eq!(
            Identifier::from_str(input).unwrap(),
            Identifier::Ipv6("2001:db8:cafe::17".parse().unwrap(), Some(Port(4711)))
        );

        let input = "\"unknown\"";
        assert_eq!(Identifier::from_str(input).unwrap(), Identifier::Unknown);

        let input = "1.1.1.1";
        assert_eq!(
            Identifier::from_str(input).unwrap(),
            Identifier::Ipv4("1.1.1.1".parse().unwrap(), None)
        );

        let input = "1.1.1.1:53";
        assert_eq!(
            Identifier::from_str(input).unwrap(),
            Identifier::Ipv4("1.1.1.1".parse().unwrap(), Some(Port(53)))
        );

        let input = "\"unclosed qoutes";
        assert!(Identifier::from_str(input).is_err());

        let input = "\"[bad::ipv6]\"";
        assert!(Identifier::from_str(input).is_err());

        let input = "bad.ipv4";
        assert!(Identifier::from_str(input).is_err());

        // Missing port
        let input = "\"[::]:\"";
        assert!(Identifier::from_str(input).is_err());

        let input = "1.1.1.1:";
        assert!(Identifier::from_str(input).is_err());
    }
}
