use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Display, Error)]
pub enum Error {
    /// Json may not be empty
    Empty,
    /// unmatched character {0:?}
    Unmatched(char),
    /// unrecognized character {0:?}
    Unrecognized(char),
    /// unexpected character {0:?} after json finished
    CharacterAfterEnd(char),
    /// {0}
    Custom(String),
}

impl<S> From<S> for Error
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self::Custom(value.into())
    }
}
