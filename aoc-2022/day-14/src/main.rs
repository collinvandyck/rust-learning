mod sand;

mod prelude {
    pub use crate::sand::*;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use prelude::*;

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let formations = read
        .lines()
        .map(|l| Formation::parse(l.unwrap()))
        .collect::<Vec<_>>();
    let cave = Cave::new(formations);
    println!("{cave}");
}
