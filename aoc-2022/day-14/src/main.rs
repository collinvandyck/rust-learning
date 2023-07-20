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
    run("input.txt");
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
    for tick in 1.. {
        if cave.tick() == Sand::Done {
            break;
        }
        if tick % 100 == 0 {
            println!("{cave}");
            println!();
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!("Grains: {}", cave.grains);
}
