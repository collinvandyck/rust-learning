use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let mut iter = file.lines();
    let mut ship = Ship::new();
    loop {
        let line = iter.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        let mut chars = line.chars();
        let mut stack_idx = 0;
        loop {
            let part: String = chars.by_ref().take(3).collect();
            if part.len() < 3 {
                // end of line
                break;
            }
            if part.starts_with('[') && part.ends_with(']') {
                let crt = Crate(part.chars().nth(1).unwrap());
                ship.insert_crate(stack_idx, crt);
            }
            // discard the space
            chars.next();
            stack_idx += 1;
        }
    }
    println!("Ship:\n{ship}");
}

#[derive(Debug)]
struct Ship(Vec<Stack>);

impl Ship {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn insert_crate(&mut self, stack_idx: usize, crt: Crate) {
        if self.0.len() <= stack_idx {
            let grow = stack_idx - self.0.len() + 1;
            for _ in 0..grow {
                self.0.push(Stack::new());
            }
        }
        let stack = self.0.get_mut(stack_idx).unwrap();
        stack.0.push_front(crt);
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        let max_stack_len = self.0.iter().map(|s| s.0.len()).max().unwrap_or(0);
        for ms in (0..max_stack_len).rev() {
            // ms is the index we will get from each stack, if it exists.
            // ..
            // iterate over each stack and print out the value
            self.0.iter().for_each(|s| match s.0.get(ms) {
                Some(crt) => buf += &format!("[{}] ", crt.0),
                None => buf += &format!("    "),
            });
            buf += "\n";
        }
        self.0
            .iter()
            .enumerate()
            .for_each(|(i, _)| buf += &format!(" {}  ", i + 1));
        buf += "\n";
        write!(f, "{buf}")
    }
}

#[derive(Debug)]
struct Stack(VecDeque<Crate>);

impl Stack {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

#[derive(Debug)]
struct Crate(char);
