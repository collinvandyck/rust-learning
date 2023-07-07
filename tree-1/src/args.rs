pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// How many levels of depth to search for files
    #[arg(short, long)]
    pub depth: Option<u32>,

    /// Show hidden files
    #[arg(short = 'H', default_value_t = false)]
    pub show_hidden: bool,

    /// The directory to search
    pub dir: Option<String>,
}
