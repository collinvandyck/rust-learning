use std::fmt::Display;

use crate::prelude::*;

pub struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new(worry_divisor: u64, mut iter: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        while let Some(monkey) = Monkey::load(worry_divisor, &mut iter) {
            monkeys.push(monkey);
            if iter.next().is_none() {
                break;
            }
        }
        Self { monkeys }
    }

    pub fn run(&mut self, rounds: usize) -> u32 {
        for round in 0..rounds {
            println!("Round: {round}");
            self.round();
        }
        self.monkey_business()
    }

    fn monkey_business(&self) -> u32 {
        let mut vals: Vec<u32> = self.monkeys.iter().map(|m| m.inspections).collect();
        vals.sort();
        vals.into_iter().rev().take(2).reduce(|a, b| a * b).unwrap()
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let moves = self.run_monkey(idx);
            for mov in moves {
                let dst = self.monkeys.get_mut(mov.idx).unwrap();
                dst.add(mov.item);
            }
        }
    }

    fn run_monkey(&mut self, idx: usize) -> Vec<SendTo> {
        let m = self.monkeys.get_mut(idx).unwrap();
        m.inspect()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self
            .monkeys
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{res}")
    }
}
