#![allow(dead_code)]
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
    //part_one("example.txt");
    //part_one("input.txt");
    test_part_one("input.txt");
}

fn test_part_one(filename: &str) {
    read_pairs(filename).iter().take(1).for_each(|pair| {
        println!("{pair}\n");
        let ordered = pair.is_ordered();
        println!("Testing: pair is ordered: {ordered}");
    })
}

fn part_one(filename: &str) {
    let pairs = read_pairs(filename);
    let sum: usize = pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| {
            if pair.is_ordered() {
                Some(idx + 1)
            } else {
                None
            }
        })
        .flatten()
        .sum();
    println!("{filename}: Sum of indices: {sum}");
}

fn read_pairs(filename: &str) -> Vec<Pair> {
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
}
