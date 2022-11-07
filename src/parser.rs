use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Copy, Clone, Debug)]
pub struct Span<'a> {
    inner: &'a str,
}

impl<'a> Span<'a> {
    pub fn new(root: &'a str) -> Self {
        Self { inner: root }
    }

    pub fn enclosed(&self, pat: char) -> Result<Self> {
        if self.inner.len() >= pat.len_utf8() * 2
            && self.inner.starts_with(pat)
            && self.inner.ends_with(pat)
        {
            Ok(Self {
                inner: &self.inner[1..self.inner.len() - 1],
            })
        } else {
            Err(Error {
                position: 0,
                expected: "\"",
            })
        }
    }

    pub fn find_enclosed(&self, pat: char) -> Option<Self> {
        let start = self.inner.find(pat)?;
        if start == self.inner.len() - 1 {
            return None;
        }
        let end = self.inner[start + 1..].find(pat)?;

        Some(Self {
            inner: &self.inner[start..end],
        })
    }

    pub fn split_once(&self, pat: char) -> (Self, Option<Self>) {
        self.inner
            .split_once(pat)
            .map(|(a, b)| (Self::new(a), Some(Self::new(b))))
            .unwrap_or((*self, None))
    }

    pub fn as_str(&self) -> &str {
        self.inner
    }

    pub fn parse<T>(&self) -> std::result::Result<T, T::Err>
    where
        T: FromStr,
    {
        self.inner.parse()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Token(&'static str);

impl Token {
    pub const DQUOTE: Self = Self("\"");
}
