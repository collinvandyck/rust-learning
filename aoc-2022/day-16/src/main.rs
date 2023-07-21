#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod args;
mod world;

mod prelude {
    pub use crate::args::*;
    pub use crate::world::*;
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

fn run(args: &Args) {
    let map = load(args);
    println!("{map:#?}");
}

fn load(args: &Args) -> Map {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let re = Regex::new(r#"Valve (\w+).*rate=(\d+);.*to valves?(.*)"#).unwrap();
    Map::new(
        read.lines()
            .map(Result::unwrap)
            .map(|l| parse_line(&l, &re))
            .map(|p| {
                let tunnels = p.tunnels.clone();
                Valve::new(p.name.to_string(), p.rate, tunnels)
            }),
    )
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// let (full, [title, year]) = re.captures(hay).unwrap().extract();
fn parse_line(line: &str, re: &Regex) -> Parsed {
    let caps = re.captures(line).unwrap();
    let (_, [name, rate, tunnels]) = caps.extract();
    let name = name.to_string();
    let rate = rate.parse::<i32>().unwrap();
    let tunnels = tunnels
        .trim()
        .split(',')
        .map(|p| p.trim().to_string())
        .collect::<Vec<_>>();
    Parsed {
        name,
        rate,
        tunnels,
    }
}

#[derive(Debug)]
struct Parsed {
    name: String,
    rate: i32,
    tunnels: Vec<String>,
}
