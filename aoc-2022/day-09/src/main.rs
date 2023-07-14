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
    run("example.txt", 13);
}

fn run(filename: &str, expected_tail_visits: usize) {
    let mut rope = Rope::new();
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    println!("{rope}");
    for line in read.lines() {
        let line = line.unwrap();
        let mov = Move::from(&line);
        rope.exec(&mov);
    }
    let tail_visits = rope.tail_visits();
    println!("Tail visits: {tail_visits}");
    assert_eq!(tail_visits, expected_tail_visits);
}
