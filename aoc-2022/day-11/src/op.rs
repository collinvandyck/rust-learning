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
    pub fn calculate(&self, old: i32) -> i32 {
        let (left, right) = self.values(old);
        match self.op {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
    fn values(&self, old: i32) -> (i32, i32) {
        (self.left.value(old), self.right.value(old))
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(i32),
}

impl Operand {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            s => Operand::Value(s.parse::<i32>().unwrap()),
        }
    }
    fn value(&self, old: i32) -> i32 {
        match self {
            Operand::Old => old,
            Operand::Value(v) => *v,
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
