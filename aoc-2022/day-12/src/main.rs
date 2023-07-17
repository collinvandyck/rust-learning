mod map;
mod tile;

mod prelude {
    pub use crate::map::*;
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
    let map = Map::from_iter(iter);
    println!("{map}");
    map.solve();
}
