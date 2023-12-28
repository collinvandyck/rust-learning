use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{collections::VecDeque, usize};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", hash_input(ex1));
    println!("p1in1 = {}", hash_input(in1));
    println!("p2ex1 = {}", init_sequence(ex1));
    println!("p2in1 = {}", init_sequence(in1));
}

fn init_sequence(input: &str) -> usize {
    let mut map = Map::new();
    for step in parse_steps(input) {
        map.accept(step);
    }
    map.focus_power()
}

struct Map {
    slots: Vec<Slot>,
}

impl Map {
    fn new() -> Self {
        Self {
            slots: (0..256).map(|_| Slot::default()).collect(),
        }
    }
    fn accept(&mut self, step: Step) {
        let slot = self
            .slots
            .get_mut(step.slot)
            .unwrap_or_else(|| panic!("no slot found for step {step:?}"));
        match step.op {
            Op::Del => slot.del(&step.label),
            Op::Set { focal_length } => slot.set(&step.label, focal_length),
        }
    }
    fn focus_power(&self) -> usize {
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.lenses.is_empty())
            .map(|(i, s)| s.focus_power(i + 1))
            .sum()
    }
}

#[derive(Default)]
struct Slot {
    lenses: VecDeque<Lens>,
}

impl Slot {
    fn del(&mut self, label: &str) {
        if let Some(idx) = self.idx(label) {
            self.lenses.remove(idx);
        }
    }
    fn set(&mut self, label: &str, focal: usize) {
        match self.idx(label) {
            Some(idx) => {
                self.lenses
                    .get_mut(idx)
                    .into_iter()
                    .for_each(|l| l.focal = focal);
            }
            None => self.lenses.push_back(Lens {
                label: label.to_string(),
                focal,
            }),
        }
    }
    fn idx(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|l| l.label == label)
    }
    fn focus_power(&self, factor: usize) -> usize {
        (1_usize..)
            .zip(self.lenses.iter())
            .map(|(sidx, lens)| sidx * lens.focal * factor)
            .sum()
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    label: String,
    op: Op,
    slot: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum Op {
    Del,
    Set { focal_length: usize },
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
                "=" => Op::Set {
                    focal_length: s[piv + 1..]
                        .parse::<usize>()
                        .expect("could not parse eq usize"),
                },
                "-" => Op::Del,
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
    fn test_pt2_in1() {
        let in1 = include_str!("in1.txt");
        let res = init_sequence(in1);
        assert_eq!(res, 269747);
    }

    #[test]
    fn test_pt2_ex1() {
        let ex1 = include_str!("ex1.txt");
        let res = init_sequence(ex1);
        assert_eq!(res, 145);
    }

    #[test]
    fn test_parse_steps() {
        let ex1 = include_str!("ex1.txt");
        let steps = parse_steps(ex1);
        assert_eq!(steps.len(), 11);
        assert_eq!(
            steps[0],
            Step {
                label: String::from("rn"),
                op: Op::Set { focal_length: 1 },
                slot: 0,
            }
        );
        assert_eq!(
            steps[1],
            Step {
                label: String::from("cm"),
                op: Op::Del,
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
