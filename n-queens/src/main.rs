use std::{fmt::Display, process};

fn main() {
    let n = 4;
    let mut board = Board::new(n);
    if board.solve() {
        println!("{}", board);
    } else {
        eprintln!("Could not solve for {n}");
        process::exit(1);
    }
}

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    vals: Vec<bool>,
}

impl Board {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            vals: vec![false; n * n],
        }
    }
    fn solve(&mut self) -> bool {
        false
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.n {
            for j in 0..self.n {
                let idx = i * self.n + j;
                if self.vals[idx] {
                    s.push_str("Q ");
                } else {
                    s.push_str("_ ");
                }
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}
