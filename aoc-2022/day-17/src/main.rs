#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod board;
mod shapes;
mod prelude {
    pub use crate::board::*;
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
    let gusts = load_gusts(args);
    dbg!(gusts);
}

fn load_gusts(args: &Args) -> Vec<Gust> {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let line = read.lines().next().unwrap().unwrap();
    line.chars().map(Into::into).collect::<Vec<_>>()
}
