mod fs;
mod shell;

mod prelude {
    pub use crate::fs::*;
    pub use crate::shell::*;
}

use prelude::*;

fn main() {
    let lines = parse_lines("example.txt");
    let mut path = Path::from("/");
    for line in lines {
        match line {
            Line::Cd(dir) => {
                path.cd(dir);
                dbg!(&path);
            }
            Line::Ls() => {}
            Line::Dir(_name) => {}
            Line::File(_size, _name) => {}
        }
    }
}

