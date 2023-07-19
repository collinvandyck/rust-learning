use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod signal;

mod prelude {
    pub use crate::signal::*;
}

use prelude::*;

fn main() {
    part_one("example.txt");
}

fn part_one(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let mut iter = read.lines().flatten();
    loop {
        let one = iter.next().unwrap();
        let two = iter.next().unwrap();
        let pair = parse_pair(one, two);
        println!("{pair:?}");
        let ordered = pair.is_ordered();
        println!("Ordered: {ordered}");
        if iter.next().is_none() {
            break;
        }
        println!();
    }
}
