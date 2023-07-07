#![warn(clippy::all, clippy::pedantic)]

mod args;
mod error;
mod walk;

mod prelude {
    pub use crate::args::*;
    pub use crate::error::*;
    pub use crate::walk::*;
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
    walk(&args, print)?;
    Ok(())
}

fn print(w: &Walked) {
    if !w.start {
        if w.depth == 0 {
            if w.last {
                print!("└─ ");
            } else {
                print!("├─ ");
            }
        } else {
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
    }
    if w.is_dir {
        println!("{}", w.name.green());
    } else if w.is_executable {
        println!("{}", w.name.red());
    } else {
        println!("{}", w.name);
    }
}
