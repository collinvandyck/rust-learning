use std::{error, fmt::Display, io};

pub type WalkResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    NotDirectory(String),
    IO(io::Error),
    NoFileName,
}

impl Error {}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(ref s) => write!(f, "{s} not found"),
            Self::IO(ref e) => write!(f, "{e}"),
            Self::NoFileName => write!(f, "no filename present"),
            Self::NotDirectory(ref s) => write!(f, "{s} is not a directory"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
