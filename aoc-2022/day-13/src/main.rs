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
    let mut pairs = vec![];
    loop {
        let one = iter.next().unwrap();
        let two = iter.next().unwrap();
        let pair = parse_pair(one, two);
        pairs.push(pair);
        if iter.next().is_none() {
            break;
        }
    }
    pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| (idx + 1, pair))
        .map(|(idx, pair)| (idx, pair, pair.is_ordered()))
        .for_each(|(idx, pair, ordered)| {
            println!("idx: {idx}");
            println!("pair: {pair:?}");
            println!("ordered: {ordered}");
        });
}
