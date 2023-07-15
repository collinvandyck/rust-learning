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

    pub fn run(&mut self) {
        // TODO: do stuff
    }
}
