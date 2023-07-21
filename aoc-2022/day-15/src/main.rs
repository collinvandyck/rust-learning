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
    part_1(&args);
}

fn part_1(args: &Args) {
    let sensors = load_sensors(args);
    sensors.iter().for_each(|s| println!("{s}"));
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
        res.push(Sensor { point, beacon })
    }
    res
}
