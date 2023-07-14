#[derive(Debug)]
pub struct Move {
    direction: Direction,
    amount: usize,
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

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
