use crate::prelude::*;

#[derive(Debug)]
pub enum Op {
    Noop,
    Addx(i64),
}

impl Op {
    /// noop
    /// addx 3
    /// addx -5
    pub fn parse(line: &str) -> Self {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[..] {
            ["noop"] => Self::Noop,
            ["addx", amt] => {
                let amt = amt.parse::<i64>().unwrap();
                Self::Addx(amt)
            }
            _ => panic!("Parse error"),
        }
    }
    pub fn cycles(&self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
    pub fn apply(&self, registers: &mut Registers) {
        match self {
            Op::Noop => {
                //println!("  noop exec");
            }
            Op::Addx(v) => {
                //let x = registers.x;
                registers.x += v;
                //println!("  addx ({v}) exec {x} -> {}", registers.x);
            }
        }
    }
}
