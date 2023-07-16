use crate::prelude::*;

#[derive(Debug)]
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
    pub fn calculate(&self, old: &mut BigNum) -> BigNum {
        match self.right {
            Operand::Old => match self.op {
                Operator::Add => old.multiply_by(2),
                Operator::Multiply => old.multiply(&old),
            },
            Operand::Value(ref v) => match self.op {
                Operator::Add => old.add(&BigNum::from(*v)),
                Operator::Multiply => old.multiply(&BigNum::from(*v)),
            },
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
