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
    let res: Option<u64> = fs
        .root
        .find(|f| match f {
            f @ Node::Dir(name, _) => {
                let size = f.total_size();
                let ok = size <= 100000;
                if ok {
                    println!("Dir {name} has total size {size}");
                }
                ok
            }
            _ => false,
        })
        .map(|mut res| {
            res = dbg!(res);
            res.iter().map(|r| r.total_size()).sum()
        });
    dbg!(res);
}
