pub use clap::Parser;

use crate::prelude::*;

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

impl Args {
    pub fn validate(&self) -> WalkResult<()> {
        if self.executables_only && self.dirs_only {
            return Err(Error::InvalidArgs(
                "executable and dirs cannot be set at the same time".into(),
            ));
        }
        Ok(())
    }
}
