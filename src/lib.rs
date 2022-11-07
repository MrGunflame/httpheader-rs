//! A crate for encoding/decoding standard HTTP headers.
pub(crate) mod error;
pub mod header;
pub(crate) mod parser;
pub(crate) mod types;

pub use parser::Parse;
