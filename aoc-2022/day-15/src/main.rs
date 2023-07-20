#![allow(dead_code)]
mod args;
mod point;

mod prelude {
    pub use crate::args::*;
    pub use crate::point::*;
    pub use clap::Parser;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use prelude::*;

fn main() {
    let args = Args::parse();
    run(&args);
}

fn run(args: &Args) {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        println!("{line}");
    }
}
