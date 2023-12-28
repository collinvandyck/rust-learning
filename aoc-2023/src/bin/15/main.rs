#![allow(dead_code, unused)]

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", init_seq(ex1));
    println!("p1in1 = {}", init_seq(in1));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    label: String,
    op: Op,
    box_idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum Op {
    Dash,
    Eq { focal_length: usize },
}

fn init_seq(input: &str) -> usize {
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
            let box_idx = hash(&label) as usize;
            let op = match &s[piv..piv + 1] {
                "=" => Op::Eq {
                    focal_length: s[piv + 1..]
                        .parse::<usize>()
                        .expect("could not parse eq usize"),
                },
                "-" => Op::Dash,
                piv => panic!("no suitable pivot: {piv}"),
            };
            Step { label, op, box_idx }
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
                box_idx: hash("rn"),
            }
        );
        assert_eq!(
            steps[1],
            Step {
                label: String::from("cm"),
                op: Op::Dash,
                box_idx: hash("cm"),
            }
        );
    }

    #[test]
    fn test_pt1_ex1() {
        let ex1 = include_str!("ex1.txt");
        let seq = init_seq(ex1);
        assert_eq!(seq, 1320);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52)
    }
}
