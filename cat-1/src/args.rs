use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The files to concatenate
    files: Vec<String>,
}
