use std::{fmt::Display, fs::write, io};

pub type WalkResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    IO(io::Error),
}

impl Error {}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(ref s) => write!(f, "Path {} not found", s),
            Self::IO(ref e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
