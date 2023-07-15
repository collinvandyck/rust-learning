use crate::prelude::*;

pub struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new(mut iter: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        while let Some(monkey) = Monkey::load(&mut iter) {
            monkeys.push(monkey);
            match iter.next() {
                Some(line) => {
                    if !line.trim().is_empty() {
                        panic!("Unexpected line: {line}");
                    }
                }
                None => break,
            }
        }
        Self { monkeys }
    }
}
