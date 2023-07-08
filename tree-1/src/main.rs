#![warn(clippy::all, clippy::pedantic)]

mod args;
mod error;
mod walk;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
    pub use crate::walk::*;
    pub use colored::Colorize;
}

use colored::Colorize;
use prelude::*;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

/// ├─ one
/// │  ├─ two
/// │  └─ three
/// │     └─ four
/// └─ five
///    └─ six
fn run() -> WalkResult<()> {
    let args = Args::parse();
    args.validate()?;
    walk(&args, print)?;
    Ok(())
}

fn print(w: &Walked) {
    if !w.start {
        // lhs tree rendering
        for v in w.lasts {
            if *v {
                print!("   ");
            } else {
                print!("│  ");
            }
        }
        // render the marker right before the name
        if w.last {
            print!("└─ ");
        } else {
            print!("├─ ");
        }
    }
    let formatted = w.details.colorize(w.name);
    println!("{}", formatted);
}
