use crate::error::Error;
use crate::parser::{Parse, Span};

/// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/ETag
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Etag<'a> {
    Weak(&'a str),
    Strong(&'a str),
}

impl<'a> Parse<'a> for Etag<'a> {
    type Error = Error;

    fn parse(s: &'a str) -> Result<Self, Self::Error> {
        let span = Span::new(s);

        match span.strip_prefix("W/") {
            Ok(span) => {
                let span = span.enclosed('"')?;
                Ok(Self::Weak(span.as_str()))
            }
            Err(_) => {
                let span = span.enclosed('"')?;
                Ok(Self::Strong(span.as_str()))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IfMatch<'a> {
    Etags(Vec<Etag<'a>>),
    Any,
}

impl<'a> Parse<'a> for IfMatch<'a> {
    type Error = Error;

    fn parse(s: &'a str) -> Result<Self, Self::Error> {
        if s == "*" {
            return Ok(Self::Any);
        }

        let mut etags = Vec::new();
        for mut val in s.split(',') {
            // Remove prefixed spaces.
            val = val.trim_start();

            etags.push(Etag::parse(val)?);
        }

        Ok(Self::Etags(etags))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IfNoneMatch<'a> {
    Etags(Vec<Etag<'a>>),
    Any,
}

impl<'a> Parse<'a> for IfNoneMatch<'a> {
    type Error = Error;

    fn parse(s: &'a str) -> Result<Self, Self::Error> {
        if s == "*" {
            return Ok(Self::Any);
        }

        let mut etags = Vec::new();
        for mut val in s.split(',') {
            // Remove prefixed spaces.
            val = val.trim_start();

            etags.push(Etag::parse(val)?);
        }

        Ok(Self::Etags(etags))
    }
}

#[cfg(test)]
mod tests {
    use super::Etag;
    use crate::parser::Parse;

    #[test]
    fn etag() {
        let input = "\"strong etag\"";
        assert_eq!(Etag::parse(input).unwrap(), Etag::Strong("strong etag"));

        let input = "W/\"weak etag\"";
        assert_eq!(Etag::parse(input).unwrap(), Etag::Weak("weak etag"));

        let input = "invalid";
        assert!(Etag::parse(input).is_err());
    }
}
