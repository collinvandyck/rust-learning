#![warn(clippy::all, clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod machine;
mod op;
mod registers;

mod prelude {
    pub use crate::machine::*;
    pub use crate::op::*;
    pub use crate::registers::*;
}

use prelude::*;

fn main() {
    //run("example-1.txt");
    run("example-2.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let ops = read
        .lines()
        .map(std::result::Result::unwrap)
        .map(|l| Op::parse(&l))
        .collect::<Vec<_>>();
    let mut machine = Machine::new(ops);
    machine.run();
}
