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
                    if !line.is_empty() {
                        panic!("line was not empty");
                    }
                }
                None => break,
            }
        }
        Self { monkeys }
    }
}
