use std::fmt::Display;

fn main() {
    let board = Board::new(4);
    println!("{}", board);
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
