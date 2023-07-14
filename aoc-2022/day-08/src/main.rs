mod forest;

mod prelude {
    pub use crate::forest::*;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use prelude::*;

fn main() {
    run("example.txt", 21);
}

fn run(filename: &str, expected_visible: u32) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let mut forest = Forest::new();
    for line in read.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let line = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .map(|c| Tree(c))
            .collect::<Vec<_>>();
        forest.add_line(line);
    }
    let visible = forest.visible();
    assert_eq!(visible, expected_visible);
}
