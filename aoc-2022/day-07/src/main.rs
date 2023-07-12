mod fs;
mod shell;

mod prelude {
    pub use crate::fs::*;
    pub use crate::shell::*;
}

use prelude::*;

fn main() {
    let mut fs = FS::new();
    let lines = parse_lines("example.txt");
    let mut iter = lines.iter().peekable();
    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        match line {
            Line::Cd(dir) => fs.cd(dir),
            Line::Ls() => loop {
                let line = iter.peek();
                match line {
                    Some(Line::Dir(_name)) => {}
                    Some(Line::File(_size, _name)) => {}
                    _ => break,
                }
                iter.next();
            },
            _ => panic!("parse error"),
        }
    }
}
