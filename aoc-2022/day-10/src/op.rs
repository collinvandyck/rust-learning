use crate::prelude::*;

#[derive(Debug)]
pub enum Op {
    Noop,
    Addx(i32),
}

impl Op {
    /// noop
    /// addx 3
    /// addx -5
    pub fn parse(line: String) -> Self {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match &parts[..] {
            &["noop"] => Self::Noop,
            &["addx", amt] => {
                let amt = amt.parse::<i32>().unwrap();
                Self::Addx(amt)
            }
            _ => panic!("Parse error"),
        }
    }
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}
