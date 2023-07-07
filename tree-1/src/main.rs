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
        first,
        parent_last,
    } = w.clone();
    if depth == 0 {
        if last {
            print!("└─ ");
        } else {
            print!("├─ ");
        }
    } else {
        if parent_last {
            print!("  {}", "  ".repeat(depth as usize));
        } else {
            print!("{}", "│  ".repeat((depth) as usize));
        }
        match (first, last) {
            (_, true) => print!("└─ "),
            _ => print!("├─ "),
        }
    }
    println!("{name} last:{last} parent_last:{parent_last}");
}
