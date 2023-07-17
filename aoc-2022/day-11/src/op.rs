use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Op {
    left: Operand,
    op: Operator,
    right: Operand,
}

impl Op {
    pub fn from(left: &str, op: &str, right: &str) -> Self {
        let left = Operand::parse(left);
        let op = Operator::parse(op);
        let right = Operand::parse(right);
        Self { left, op, right }
    }
    pub fn calculate(&self, old: &mut BigNum) {
        match self.right {
            Operand::Old => match self.op {
                Operator::Add => old.multiply_by(2),
                Operator::Multiply => old.square(),
            },
            Operand::Value(v) => match self.op {
                Operator::Add => old.add_num(v),
                Operator::Multiply => old.multiply_by(v),
            },
        };
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Value(u64),
}

impl Operand {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            s => {
                let vu64 = s.parse::<u64>().unwrap();
                Operand::Value(vu64)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Invalid operator: {s}"),
        }
    }
}
