#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod args;
mod graph;
mod solver;
mod valve;
mod world;

mod prelude {
    pub use crate::args::*;
    pub use crate::graph::*;
    pub use crate::solver::*;
    pub use crate::valve::*;
    pub use crate::world::*;
    pub use clap::Parser;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use prelude::*;
use regex::Regex;

fn main() {
    let args = Args::parse();
    run(&args);
}

fn run(args: &Args) {
    let map = load(args);
    let mut solver = Solver::new(args, map);
    let now = Instant::now();
    let score = solver.solve();
    let elapsed = now.elapsed();
    println!(
        "Score: {score} Dur: {}.{}s",
        elapsed.as_secs(),
        elapsed.as_millis()
    );
}

fn load(args: &Args) -> Map {
    load_map(&args.filename)
}

pub fn load_map(filename: &str) -> Map {
    let file = File::open(filename).unwrap();
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
