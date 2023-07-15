use crate::prelude::*;

pub struct Game {
    monkeys: Vec<Monkey>,
}

impl Game {
    pub fn new(iter: impl Iterator<Item = String>) -> Self {
        let monkeys = vec![];
        Self { monkeys }
    }
}
