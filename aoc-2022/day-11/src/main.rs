#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod game;
mod item;
mod monkey;
mod op;
mod test;

mod prelude {
    pub use crate::game::*;
    pub use crate::item::*;
    pub use crate::monkey::*;
    pub use crate::op::*;
    pub use crate::test::*;
}

use prelude::*;

fn main() {
    run_example("example.txt");
}

fn run_example(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let _game = Game::new(read.lines().map(std::result::Result::unwrap));
}
