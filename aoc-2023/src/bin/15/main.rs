#![allow(dead_code, unused)]

use std::collections::VecDeque;

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", hash_input(ex1));
    println!("p1in1 = {}", hash_input(in1));
}

fn init_sequence(input: &str) -> usize {
    let map = Map::new();
    for step in parse_steps(input) {
        let Step { label, op, slot } = step;
    }
    todo!()
}

struct Map {
    slots: Vec<Slot>,
}

impl Map {
    fn new() -> Self {
        Self {
            slots: (0..255).map(|_| Slot::default()).collect(),
        }
    }
}

#[derive(Default)]
struct Slot {
    lenses: VecDeque<Lens>,
}

struct Lens {
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    label: String,
    op: Op,
    slot: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum Op {
    Dash,
    Eq { focal_length: usize },
}

fn hash_input(input: &str) -> usize {
    parse(input)
        .iter()
        .par_bridge()
        .into_par_iter()
        .map(|s| hash(s))
        .sum()
}

fn hash(s: &str) -> usize {
    let mut val = 0;
    for ch in s.chars() {
        let code = ch as u32;
        val += code;
        val *= 17;
        val = val % 256;
    }
    val as usize
}

fn parse_steps(input: &str) -> Vec<Step> {
    input
        .trim()
        .split(",")
        .map(|s| {
            let piv = s.find(|c| c == '=' || c == '-').unwrap_or_default();
            let label = s[0..piv].to_string();
            let slot = hash(&label) as usize;
            let op = match &s[piv..piv + 1] {
                "=" => Op::Eq {
                    focal_length: s[piv + 1..]
                        .parse::<usize>()
                        .expect("could not parse eq usize"),
                },
                "-" => Op::Dash,
                piv => panic!("no suitable pivot: {piv}"),
            };
            Step { label, op, slot }
        })
        .collect()
}

fn parse(input: &str) -> Vec<String> {
    input.trim().split(",").map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_steps() {
        let ex1 = include_str!("ex1.txt");
        let steps = parse_steps(ex1);
        assert_eq!(steps.len(), 11);
        assert_eq!(
            steps[0],
            Step {
                label: String::from("rn"),
                op: Op::Eq { focal_length: 1 },
                slot: 0,
            }
        );
        assert_eq!(
            steps[1],
            Step {
                label: String::from("cm"),
                op: Op::Dash,
                slot: hash("cm"),
            }
        );
    }

    #[test]
    fn test_pt1_ex1() {
        let ex1 = include_str!("ex1.txt");
        let seq = hash_input(ex1);
        assert_eq!(seq, 1320);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52)
    }
}
