pub use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// How many levels of depth to search for files
    #[arg(short, long)]
    pub depth: Option<u32>,

    /// Show hidden files
    #[arg(short = 'H', default_value_t = false)]
    pub show_hidden: bool,

    /// Only show directories
    #[arg(short = 'D', default_value_t = false)]
    pub dirs_only: bool,

    /// Only show executables
    #[arg(short = 'E', default_value_t = false)]
    pub executables_only: bool,

    /// The directory to search
    pub dir: Option<String>,
}
