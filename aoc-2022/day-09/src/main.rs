#![warn(clippy::all, clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod mov;
mod rope;

mod prelude {
    pub use crate::mov::*;
    pub use crate::rope::*;
}

use prelude::*;

fn main() {
    run("example.txt", 2, Some(13));
    run("example.txt", 10, Some(1));
    run("example-2.txt", 10, Some(36));
    run("input.txt", 10, Some(2482));
}

fn run(filename: &str, num_knots: usize, expected_tail_visits: Option<usize>) {
    let mut rope = Rope::new(num_knots);
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        let mov = Move::from(&line);
        rope.exec(&mov);
    }
    let tail_visits = rope.tail_visits();
    println!("Tail visits: {tail_visits}");
    if let Some(expected) = expected_tail_visits {
        assert_eq!(tail_visits, expected);
    }
}
