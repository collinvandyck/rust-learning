use std::{fmt::Display, vec::IntoIter};

use crate::prelude::*;

pub struct Machine {
    registers: Registers,
    ops: IntoIter<Op>,
    cur: Option<OpExec>,
}

pub struct State<'a> {
    pub tick: i64,
    pub registers: &'a Registers,
}

impl Machine {
    pub fn new(ops: Vec<Op>) -> Self {
        let registers = Registers::new();
        let ops = ops.into_iter();
        let cur = None;
        Self {
            registers,
            ops,
            cur,
        }
    }
    // runs to completion, when there is no more work to be done
    pub fn run<F>(&mut self, mut callback: F)
    where
        F: FnMut(State),
    {
        if !self.load() {
            return;
        }
        for tick in 1_i64.. {
            // load an operation if we don't have one yet
            if !self.load() {
                break;
            }
            //println!("STR Tick: {tick} registers: {:?}", self.registers);
            callback(State {
                tick,
                registers: &self.registers,
            });

            // execute the current operation.
            if let Some(mut exec) = self.cur.take() {
                // if we still need to wait, return early
                if exec.dec_wait() {
                    self.cur = Some(exec);
                } else {
                    // the exec is done.
                    exec.apply(&mut self.registers);
                }
            }
            //println!("END Tick: {tick} registers: {:?}", self.registers);
        }
    }
    // loads the next op if necessary, and returns true if we have an op
    fn load(&mut self) -> bool {
        if self.cur.is_none() {
            // there is no operation. load the next op.
            if let Some(op) = self.ops.next() {
                //println!("Loaded: {op:?}");
                let cycles = op.cycles();
                let exec = OpExec { op, cycles };
                self.cur = Some(exec);
            }
        }
        self.cur.is_some()
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

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
