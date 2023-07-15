use std::vec::IntoIter;

use crate::prelude::*;

pub struct Machine {
    registers: Registers,
    ops: IntoIter<Op>,
    cur: Option<OpExec>,
}

impl Machine {
    pub fn new(ops: Vec<Op>) -> Self {
        let registers = Registers::new();
        let ops = ops.into_iter();
        let cur = None;
        Self {
            registers,
            cur,
            ops,
        }
    }
    // runs to completion, when there is no more work to be done
    pub fn run(&mut self) {
        for tick in (1 as u64).. {
            println!("Tick: {tick}");

            // if we have a current operation, move it forward
            if let Some(mut exec) = self.cur.take() {
                // if we still need to wait, return early
                if exec.dec_wait() {
                    self.cur = Some(exec);
                    continue;
                }
                // the exec is done.
                exec.apply(&mut self.registers);
            }
            match self.ops.next() {
                Some(op) => {
                    let cycles = op.cycles();
                    let exec = OpExec { op, cycles };
                    self.cur = Some(exec);
                }
                None => break,
            }
        }
    }
}

struct OpExec {
    op: Op,
    cycles: i32,
}

impl OpExec {
    fn dec_wait(&mut self) -> bool {
        self.cycles -= 1;
        self.cycles > 0
    }
    fn apply(&self, registers: &mut Registers) {
        self.op.apply(registers);
    }
}
