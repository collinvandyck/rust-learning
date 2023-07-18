#![warn(clippy::all, clippy::pedantic)]

mod game;
mod map;
mod point;
mod tile;

mod prelude {
    pub use crate::game::*;
    pub use crate::map::*;
    pub use crate::point::*;
    pub use crate::tile::*;
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

use prelude::*;

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let iter = read.lines().map(result::Result::unwrap);
    let game = Game::from_iter(iter);
    println!("{game}");
    game.solve();
}
