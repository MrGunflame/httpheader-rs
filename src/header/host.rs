use crate::error::Error;
use crate::parser::{Parse, Span};
use crate::types::Port;

/// The `Host` header.
///
/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Host for more information.
pub struct Host<'a> {
    pub host: &'a str,
    pub port: Option<Port>,
}

impl<'a> Parse<'a> for Host<'a> {
    type Error = Error;

    fn parse(s: &'a str) -> Result<Self, Self::Error> {
        let span = Span::new(s);
        let (host, port) = span.split_once(':');

        let host = host.as_str();
        let port = match port {
            Some(port) => Some(port.parse()?),
            None => None,
        };

        Ok(Self { host, port })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parse;

    use super::Host;

    #[test]
    fn host() {
        let input = "mozilla.org";
        let host = Host::parse(input).unwrap();
        assert_eq!(host.host, "mozilla.org");

        let input = "mozilla.org:80";
        let host = Host::parse(input).unwrap();
        assert_eq!(host.host, "mozilla.org");
        assert_eq!(host.port, Some(80.into()));

        let input = "mozilla.org:invalid";
        assert!(Host::parse(input).is_err());
    }
}
