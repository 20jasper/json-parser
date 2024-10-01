use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Display, Error)]
pub enum Error {
    /// Json may not be empty
    Empty,
    /// unmatched character {0:?}
    Unmatched(char),
    /// {0}
    Custom(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.into())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}
