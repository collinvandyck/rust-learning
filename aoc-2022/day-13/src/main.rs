#![allow(dead_code)]
use std::{
    cmp::Ordering,
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
    part_two("example.txt");
    part_one("input.txt");
    part_two("input.txt");
}

fn part_two(filename: &str) {
    let mut packets = read_pairs(filename)
        .into_iter()
        .flat_map(|p| vec![p.left, p.right])
        .collect::<Vec<_>>();
    let d1 = parse_packet("[[2]]".to_string());
    let d2 = parse_packet("[[6]]".to_string());
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort_by(|a, b| a.cmp(b));
    let decoder_key = packets
        .iter()
        .enumerate()
        .filter(|(_idx, pkt)| {
            // they have to match a divider packet
            pkt.cmp(&d1) == Ordering::Equal || pkt.cmp(&d2) == Ordering::Equal
        })
        .map(|(idx, _)| idx + 1)
        .take(2)
        .reduce(|one, two| one * two)
        .unwrap();
    println!("Decoder key: {decoder_key}");
}

fn part_one(filename: &str) {
    let pairs = read_pairs(filename);
    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            if pair.is_ordered() {
                Some(idx + 1)
            } else {
                None
            }
        })
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
