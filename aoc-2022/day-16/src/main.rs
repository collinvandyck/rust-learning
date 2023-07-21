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
    let valves = load(args);
    for valve in valves {
        println!("{valve:?}");
    }
}

fn load(args: &Args) -> Vec<Valve> {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let re = Regex::new(r#"Valve (\w+).*rate=(\d+);.*to valves?(.*)"#).unwrap();
    let parsed = read
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_line(l, &re))
        .collect::<Vec<_>>();
    let valves = parsed
        .iter()
        .map(|p| Valve::new(p.name.to_string(), p.rate))
        .collect::<Vec<_>>();
    parsed.iter().for_each(|parsed| {
        // do stuff
        let source = &parsed.name;
        parsed.tunnels.iter().for_each(|dest| {
            println!("Adding valve path {source} -> {dest}");
            // add this
        })
    });
    valves
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
fn parse_line(line: String, re: &Regex) -> Parsed {
    let caps = re.captures(&line).unwrap();
    let valve = caps.get(1).unwrap().as_str();
    let rate = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let tunnels = caps
        .get(3)
        .unwrap()
        .as_str()
        .trim()
        .split(',')
        .map(|p| p.trim().to_string())
        .collect::<Vec<_>>();
    Parsed {
        name: valve.to_string(),
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
