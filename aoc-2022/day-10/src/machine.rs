use crate::prelude::*;

pub struct Machine {
    registers: Registers,
}

impl Machine {
    pub fn new(ops: Vec<Op>) -> Self {
        let registers = Registers::new();
        Self { registers }
    }
}
