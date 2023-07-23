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
    let network = load_network("nano.txt");
    let mut state = State::new(&network, "AA".into());
    let score = state.solve();
    println!("Score: {score}");
}

fn load_network(filename: &str) -> Network {
    let parser = Parser::new();
    let input = fs::read(filename).unwrap();
    let input = BufReader::new(input.as_slice());
    let network = Network::new(input.lines().map(Result::unwrap).map(|l| parser.valve(&l)));
    network
}
