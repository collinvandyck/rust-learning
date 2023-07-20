#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod args;
mod sand;

mod prelude {
    pub use crate::args::*;
    pub use crate::sand::*;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use prelude::*;

fn main() {
    let args = Args::parse();
    match &args.part {
        Some(1) | None => part_one(&args),
        Some(2) => part_two(&args),
        Some(p) => panic!("Unknown part: {}", p),
    }
}

fn part_two(args: &Args) {
    let formations = load_formations(args);
    let mut cave = Cave::new(&formations);
    cave.use_floor(true);
    println!("{cave}");
    for tick in 1.. {
        if cave.tick() == Sand::Done {
            println!("{cave}");
            println!("Ticks: {tick}");
            break;
        }
        if tick % 100 == 0 {}
    }
    println!("Grains: {}", cave.grains);
}

fn part_one(args: &Args) {
    let formations = load_formations(args);
    let mut cave = Cave::new(&formations);
    println!("{cave}");
    for tick in 1.. {
        if cave.tick() == Sand::Done {
            println!("{cave}");
            println!("Ticks: {tick}");
            break;
        }
        if tick % 100 == 0 {}
    }
    println!("Grains: {}", cave.grains);
}

fn load_formations(args: &Args) -> Vec<Formation> {
    let file = File::open(&args.filename).unwrap();
    let read = BufReader::new(file);
    read.lines()
        .map(|l| Formation::parse(&l.unwrap()))
        .collect::<Vec<_>>()
}
