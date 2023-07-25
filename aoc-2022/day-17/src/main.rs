#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod board;
mod gust;
mod shapes;
mod prelude {
    pub use crate::board::*;
    pub use crate::gust::*;
    pub use crate::shapes::*;
    pub use clap::Parser;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    pub filename: String,
}

fn main() {
    let args = &Args::parse();
    run(args);
}

fn run(args: &Args) {
    let board = Board::new();
    let gusts: Gusts = load_gusts(args);
    let shapes: Shapes = shapes();
}

fn load_gusts(args: &Args) -> Gusts {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let line = read.lines().next().unwrap().unwrap();
    let gs = line.chars().map(Into::into).collect::<Vec<_>>();
    gs.into()
}
