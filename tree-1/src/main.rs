#![warn(clippy::all, clippy::pedantic)]

mod args;
mod error;
mod walk;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
    pub use crate::walk::*;
}

use prelude::*;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> WalkResult<()> {
    let args = Args::parse();
    let start = args.dir.unwrap_or(".".into());
    let mut paths = vec![];
    walk(&start, args.depth, |p| {
        println!("{p}");
        paths.push(p);
    })
}
