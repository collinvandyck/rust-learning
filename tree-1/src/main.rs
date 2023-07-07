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

/// ├─ one
/// │  ├─ two
/// │  └─ three
/// │     └─ four
/// └─ five
///    └─ six
fn run() -> WalkResult<()> {
    let args = Args::parse();
    let start = args.dir.unwrap_or(".".into());
    walk(&start, args.depth, print)?;
    Ok(())
}

fn print(w: Walked) {
    let Walked {
        name,
        depth,
        last,
        first: _,
        parent_last,
    } = w.clone();
    if depth == 0 {
        if last {
            print!("└─ ");
        } else {
            print!("├─ ");
        }
    } else {
        // lhs tree rendering
        if parent_last {
            if depth > 1 {
                // render the bars for the parents up to depth-1 times.
                print!("{}", "│  ".repeat((depth - 1) as usize));
                print!("  {}", "  ".repeat(depth as usize));
            } else {
                print!("  {}", "  ".repeat(depth as usize));
            }
        } else {
            print!("{}", "│  ".repeat((depth) as usize));
        }
        // render the marker right before the name
        if last {
            print!("└─ ");
        } else {
            print!("├─ ");
        }
    }
    println!("{name} (parent_last:{parent_last})");
}
