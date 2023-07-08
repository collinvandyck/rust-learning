pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The files to concatenate
    pub files: Vec<String>,
}
