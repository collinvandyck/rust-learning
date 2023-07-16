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
    pub fn calculate(&self, old: &BigNum) -> BigNum {
        let (left, right) = self.values(old);
        match self.op {
            Operator::Add => left.add(right),
            Operator::Multiply => left.multiply(right),
        }
    }
    fn values<'a>(&'a self, old: &'a BigNum) -> (&'a BigNum, &'a BigNum) {
        (self.left.value(old), self.right.value(old))
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(BigNum),
}

impl Operand {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            s => {
                let vu64 = s.parse::<u64>().unwrap();
                Operand::Value(BigNum::from(vu64))
            }
        }
    }
    fn value<'a>(&'a self, old: &'a BigNum) -> &'a BigNum {
        match self {
            Operand::Old => old,
            Operand::Value(v) => v,
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
