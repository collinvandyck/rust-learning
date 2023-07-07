pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// How many levels of depth to search for files
    #[arg(short, long)]
    pub depth: Option<u32>,

    /// The directory to search
    pub dir: String,
}
