#![warn(clippy::all, clippy::pedantic)]

mod args;
mod map;
mod point;
mod sensor;

mod prelude {
    pub use crate::args::*;
    pub use crate::map::*;
    pub use crate::point::*;
    pub use crate::sensor::*;
    pub use clap::Parser;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

use prelude::*;
use regex::Regex;

fn main() {
    let args = Args::parse();
    part_1(&args);
    part_2(&args);
}

fn part_1(args: &Args) {
    println!("Part 1...");
    let map = load_map(args);
    println!("Searching for beacon placements.");
    let val = map.beacon_not_possible(args.y);
    println!("At y={} the beacon cannot be in {} places.", args.y, val);
}

fn part_2(args: &Args) {
    println!("Part 2...");
    let map = load_map(args);
    let (min, max) = part_2_bounds(args);
    println!("Min: {min} Max: {max}");
    let beacon = map.find_beacon(min, max);
    println!("Beacon found at {beacon}");
    let tuning_frequency = beacon.0 * 4000000 + beacon.1;
    println!("Tuning frequency: {tuning_frequency}");
}

fn part_2_bounds(args: &Args) -> (i32, i32) {
    match args.filename.as_str() {
        "example.txt" => (0, 20),
        "input.txt" => (0, 4000000),
        filename => {
            eprintln!("Unexpected filename: {filename}");
            process::exit(1);
        }
    }
}

fn load_map(args: &Args) -> Map {
    let sensors = load_sensors(args);
    let (min, max) = Point::min_max(sensors.iter().flat_map(Sensor::bounds)).unwrap();
    let res = Map::new(sensors, min, max);
    if args.print_map {
        println!("{res}");
    }
    res
}

fn load_sensors(args: &Args) -> Vec<Sensor> {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    let re = Regex::new(r#".*x=(-?\w+).*y=(-?\w+).*x=(-?\w+).*y=(-?\w+).*"#).unwrap();
    let mut res = vec![];
    for line in read.lines() {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let mut caps = caps
            .iter()
            .flatten()
            .skip(1)
            .take(4)
            .map(|s| s.as_str().parse::<i32>().unwrap());
        let point = Point(caps.next().unwrap(), caps.next().unwrap());
        let beacon = Point(caps.next().unwrap(), caps.next().unwrap());
        res.push(Sensor { point, beacon });
    }
    res
}
