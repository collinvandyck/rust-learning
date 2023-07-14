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
    run("example.txt");
}

fn run(filename: &str) {
    let mut rope = Rope::new();
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    println!("{rope}");
    for line in read.lines() {
        let line = line.unwrap();
        let mov = Move::from(&line);
        println!("{mov:?}");
        rope.exec(&mov);
        println!("{rope}");
    }
}
