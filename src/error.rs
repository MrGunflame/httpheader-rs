use crate::parser::Token;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    pub(crate) position: usize,
    pub(crate) expected: &'static str,
}
