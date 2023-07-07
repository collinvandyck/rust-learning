use std::fmt::Display;

pub type WalkResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound(String),
}

impl Error {}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(ref s) => write!(f, "Path {} not found", s),
        }
    }
}
