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
    run("example.txt", 95437, 24933642);
    run("input.txt", 1182909, 2832508);
}

fn run(filename: &str, part_1_answer: u64, part_2_answer: u64) {
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

    // part one
    let size_of_chosen_dirs: u64 = fs
        .into_iter()
        .filter(|f| match f {
            f @ Node::Dir(_, _) => f.total_size() <= 100000,
            _ => false,
        })
        .map(|node| node.total_size())
        .sum();
    assert_eq!(size_of_chosen_dirs, part_1_answer);

    // part two
    let capacity = 70_000_000 as u64;
    let usage = fs.root.total_size();
    let free = capacity - usage;
    let desired_free = 30_000_000 as u64;
    let needs_free = if free >= desired_free {
        0
    } else {
        desired_free - free
    };
    println!("{filename}: cap: {capacity} usage: {usage} free: {free} needs_free: {needs_free}");
    // find the smallest total sized directory which is at least needs_free
    let mut candidates = fs
        .into_iter()
        .filter(|f| match f {
            f @ Node::Dir(_, _) => f.total_size() >= needs_free,
            _ => false,
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|f| f.total_size());
    let res = candidates.get(0).unwrap().total_size();
    println!("The candidate directory to be deleted has size {res}");
    assert_eq!(res, part_2_answer);
}
