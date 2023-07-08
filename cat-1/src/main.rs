mod args;
mod error;
mod input;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
    pub use crate::input::*;
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
    let input = Input::new(args)?;
    consume(args, input)?;
    Ok(())
}

fn consume(args: &Args, input: Input) -> CatResult<()> {
    Ok(())
}
