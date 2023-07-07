#![warn(clippy::all, clippy::pedantic)]

mod args;

mod prelude {
    pub use crate::args::*;
}

use prelude::*;

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
