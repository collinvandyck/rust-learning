#![allow(dead_code)]
mod args;
mod point;
mod sensor;

mod prelude {
    pub use crate::args::*;
    pub use crate::point::*;
    pub use crate::sensor::*;
    pub use clap::Parser;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use prelude::*;
use regex::Regex;

fn main() {
    let args = Args::parse();
    run(&args);
}

fn run(args: &Args) {}

fn load_sensors(args: &Args) -> Vec<Sensor> {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let re = Regex::new(r#"x=(\w+).*y=(\w+).*x=(\w+).*y=(\w+)"#).unwrap();
    let res = vec![];
    for line in read.lines() {
        let line = line.unwrap();
        println!("{line}");
    }
    res
}
