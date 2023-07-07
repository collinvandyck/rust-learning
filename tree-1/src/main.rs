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
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let start = args.dir.unwrap_or(".".into());
    let mut paths = vec![];
    walk(&start, |p| {
        println!("path: {p:?}");
        paths.push(p);
    })?;
    println!("Paths: {paths:?}");
    Ok(())
}
