use std::fmt::Display;

use crate::prelude::*;

pub struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new(mut iter: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        while let Some(monkey) = Monkey::load(&mut iter) {
            monkeys.push(monkey);
            if iter.next().is_none() {
                break;
            }
        }
        Self { monkeys }
    }

    pub fn run(&mut self, rounds: usize) -> u32 {
        for _ in 0..rounds {
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
            self.run_monkey(idx).into_iter().for_each(|send| {
                let dst = self.monkeys.get_mut(send.idx);
                if dst.is_none() {
                    panic!("Moneky does not exist at {}", send.idx);
                }
                dst.unwrap().add(send.item);
            })
        }
    }

    fn run_monkey(&mut self, idx: usize) -> Vec<SendTo> {
        self.monkeys
            .get_mut(idx)
            .iter_mut()
            .flat_map(|m| m.inspect())
            .collect()
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
