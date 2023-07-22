#![warn(clippy::all, clippy::pedantic)]
#![allow(dead_code)]

mod graph;
mod model;

mod prelude {
    pub use crate::graph::*;
    pub use crate::model::*;
}

use std::{
    fs,
    io::{BufRead, BufReader},
};

use prelude::*;

fn main() {
    let network = load_network("example.txt");
    let state = State::new(&network, "AA".into());
    for mov in state.moves() {
        println!("AA: {}", mov);
    }
}

fn load_network(filename: &str) -> Network {
    let parser = Parser::new();
    let input = fs::read(filename).unwrap();
    let input = BufReader::new(input.as_slice());
    let network = Network::new(input.lines().map(Result::unwrap).map(|l| parser.valve(&l)));
    network
}
