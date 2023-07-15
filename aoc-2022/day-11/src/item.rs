use crate::prelude::*;

#[derive(Debug)]
pub struct Item {
    worry: i32,
}

impl Item {
    pub fn new(worry: i32) -> Self {
        Self { worry }
    }
    pub fn inspect(&mut self, op: &Op) {
        // first, mutate the worry value on inspection
        self.worry = op.calculate(self.worry);
    }
}
