use crate::error::Error;
use crate::parser::Span;
use crate::types::Port;
use crate::Parse;

/// The `Origin` header.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Origin for more information.
#[derive(Clone, Debug)]
pub enum Origin<'a> {
    Null,
    Origin {
        scheme: &'a str,
        hostname: &'a str,
        port: Option<Port>,
    },
}

impl<'a> Parse<'a> for Origin<'a> {
    type Error = Error;

    fn parse(s: &'a str) -> Result<Self, Self::Error> {
        let mut span = Span::new(s);

        if s == "null" {
            return Ok(Self::Null);
        }

        let scheme = span.split_off("://")?.as_str();
        let (hostname, port) = span.split_once(':');
        let port = match port {
            Some(port) => Some(port.parse()?),
            None => None,
        };

        Ok(Self::Origin {
            scheme,
            hostname: hostname.as_str(),
            port,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;

    use super::Origin;

    #[test]
    fn origin() {
        let input = "http://mozilla.org:80";
        let origin = Origin::parse(input).unwrap();
        match origin {
            Origin::Origin {
                scheme,
                hostname,
                port,
            } => {
                assert_eq!(scheme, "http");
                assert_eq!(hostname, "mozilla.org");
                assert_eq!(port, Some(80.into()));
            }
            _ => panic!(),
        }

        let input = "https://localhost";
        let origin = Origin::parse(input).unwrap();
        match origin {
            Origin::Origin {
                scheme,
                hostname,
                port,
            } => {
                assert_eq!(scheme, "https");
                assert_eq!(hostname, "localhost");
                assert_eq!(port, None);
            }
            _ => panic!(),
        }

        let input = "null";
        let origin = Origin::parse(input).unwrap();
        assert!(matches!(origin, Origin::Null));
    }
}
