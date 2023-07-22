#![warn(clippy::all, clippy::pedantic)]

mod model;

mod prelude {
    pub use crate::model::*;
}

use std::{
    fs,
    io::{BufRead, BufReader},
};

use prelude::*;

fn main() {
    let parser = Parser::new();
    let input = fs::read("example.txt").unwrap();
    let input = BufReader::new(input.as_slice());
    for line in input.lines() {
        let line = line.unwrap();
        let valve = parser.valve(&line);
        println!("{valve}");
    }
}
