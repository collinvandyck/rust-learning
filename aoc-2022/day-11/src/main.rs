#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod bignum;
mod game;
mod item;
mod monkey;
mod num;
mod op;
mod test;

mod prelude {
    pub use crate::bignum::*;
    pub use crate::game::*;
    pub use crate::item::*;
    pub use crate::monkey::*;
    pub use crate::num::*;
    pub use crate::op::*;
    pub use crate::test::*;
}

use prelude::*;

fn main() {
    //run_example(3, 20, "input.txt");
    run_example(1, 1000, "input.txt");
}

fn run_example(worry_divisor: u64, rounds: usize, filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let iter = read.lines().map(std::result::Result::unwrap);
    let mut game = Game::new(worry_divisor, iter);
    //println!("Start:\n{game}");
    let monkey_business = game.run(rounds);
    //println!("Finish:\n{game}");
    println!("Monkey business: {monkey_business}");
}
