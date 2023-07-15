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
    part_one("example-1.txt");
    part_one("example-2.txt");
    part_one("input.txt");
}

fn part_one(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let ops = read
        .lines()
        .map(std::result::Result::unwrap)
        .map(|l| Op::parse(&l))
        .collect::<Vec<_>>();
    let mut machine = Machine::new(ops);
    let mut sum = 0_i64;
    machine.run(|s| {
        if s.tick == 20 || (s.tick + 20) % 40 == 0 {
            let strength = s.tick * s.registers.x;
            println!(
                "Callback tick:{} registers:{:?} strength:{}",
                s.tick, s.registers, strength
            );
            sum += strength;
        }
    });
    println!("Sum of signal strengths: {sum}");
}
