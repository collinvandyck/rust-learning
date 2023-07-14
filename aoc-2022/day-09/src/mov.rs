use std::fmt::Display;

#[derive(Debug)]
pub struct Move {
    pub direction: Direction,
    pub amount: usize,
}

impl Move {
    pub fn from(s: &str) -> Self {
        let parts = s.split(' ').filter(|s| s != &" ").collect::<Vec<&str>>();
        match &parts[..] {
            &[dir, n] => {
                let n = n.parse::<usize>().unwrap();
                match dir {
                    "R" => Self {
                        direction: Direction::Right,
                        amount: n,
                    },
                    "L" => Self {
                        direction: Direction::Left,
                        amount: n,
                    },
                    "U" => Self {
                        direction: Direction::Up,
                        amount: n,
                    },
                    "D" => Self {
                        direction: Direction::Down,
                        amount: n,
                    },
                    _ => panic!("Unknown direction: {dir}"),
                }
            }
            _ => panic!("parse error: #{parts:?}"),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.direction, self.amount)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
