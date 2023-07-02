use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not bind to port: {0}")]
    BindFailure(io::Error),

    #[error("Accept failure: {0}")]
    AcceptFailure(io::Error),
}
