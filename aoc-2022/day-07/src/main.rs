mod fs;
mod shell;

mod prelude {
    pub use crate::fs::*;
    pub use crate::shell::*;
}

use prelude::*;

fn main() {
    let lines = parse_lines("example.txt");
    dbg!(lines);
}

