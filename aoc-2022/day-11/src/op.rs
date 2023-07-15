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
}

#[derive(Debug)]
enum Operand {
    Old,
    New,
    Value(i32),
}

impl Operand {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            "new" => Operand::New,
            s => Operand::Value(s.parse::<i32>().unwrap()),
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
