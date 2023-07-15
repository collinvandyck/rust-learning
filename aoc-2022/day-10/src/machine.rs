use std::vec::IntoIter;

use crate::prelude::*;

pub struct Machine {
    registers: Registers,
    ops: IntoIter<Op>,
}

impl Machine {
    pub fn new(ops: Vec<Op>) -> Self {
        let registers = Registers::new();
        Self {
            registers,
            ops: ops.into_iter(),
        }
    }
    // runs to completion, when there is no more work to be done
    pub fn run(&mut self) {
        while let Some(op) = self.ops.next() {
            dbg!(op);
        }
    }
}
