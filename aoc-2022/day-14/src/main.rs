#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod sand;

mod prelude {
    pub use crate::sand::*;
}
use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread,
    time::Duration,
};

use prelude::*;

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let formations = read
        .lines()
        .map(|l| Formation::parse(&l.unwrap()))
        .collect::<Vec<_>>();
    let mut cave = Cave::new(&formations);
    println!("{cave}");
    for x in 0..500 {
        if cave.tick() == Sand::Done {
            break;
        }
        println!("{cave}");
        if x < 60 - 1 {
            println!();
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!("Grains: {}", cave.grains);
}
