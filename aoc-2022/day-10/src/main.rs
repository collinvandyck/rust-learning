use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod machine;
mod op;

mod prelude {
    pub use crate::machine::*;
    pub use crate::op::*;
}

use prelude::*;

fn main() {
    run("example-1.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let ops = read
        .lines()
        .map(|l| l.unwrap())
        .map(Op::parse)
        .collect::<Vec<_>>();
    let mut machine = Machine::new(ops);
}
