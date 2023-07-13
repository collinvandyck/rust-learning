mod fs;
mod path;
mod shell;

mod prelude {
    pub use crate::fs::*;
    pub use crate::path::*;
    pub use crate::shell::*;
}

use prelude::*;
fn main() {
    run("example.txt");
    run("input.txt");
}

fn run(filename: &str) {
    let mut fs = FS::new();
    let lines = parse_lines(filename);
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
                match iter.peek() {
                    Some(Line::Dir(name)) => Some(Node::Dir(name.to_string(), vec![])),
                    Some(Line::File(size, name)) => Some(Node::File(name.to_string(), *size)),
                    _ => break,
                }
                .into_iter()
                .for_each(|f| fs.add(f));
                iter.next();
            },
            _ => panic!("parse error"),
        }
    }
    println!("{filename}: total usage: {}", fs.root.total_size());
    let res: u64 = fs
        .root
        .find(|f| match f {
            f @ Node::Dir(_, _) => f.total_size() <= 100000,
            _ => false,
        })
        .into_iter()
        .map(|node| node.total_size())
        .sum();

    println!("{filename}: {res}");
}
