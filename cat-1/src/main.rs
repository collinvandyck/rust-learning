#![warn(clippy::all, clippy::pedantic)]

mod args;
mod error;
mod input;
mod iter;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
    pub use crate::input::*;
    pub use crate::iter::*;
}

use std::process;

use prelude::*;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> CatResult<()> {
    let input = BetterIterator::new(args)?;
    for line in input {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}
