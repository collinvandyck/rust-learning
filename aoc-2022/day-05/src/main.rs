use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Deref, Index},
};

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);
    let mut iter = file.lines();
    loop {
        let line = iter.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        let mut chars = line.chars();
        let mut stack_idx = 0;
        // loop over the entries in the line. we'll add items into
        // this vector, and if it's not empty at the end we'll add them
        // into the ship.
        let mut ship = Ship::new();
        loop {
            let part: String = chars.by_ref().take(3).collect();
            if part.len() < 3 {
                // end of line
                println!("Done");
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
}

struct Ship(Vec<Stack>);

impl Ship {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn insert_crate(&mut self, stack_idx: usize, crt: Crate) {
        if self.0.len() <= stack_idx {
            let grow = stack_idx - self.0.len() + 1;
            dbg!(grow);
            for _ in 0..grow {
                self.0.push(Stack::new());
            }
        }
        let stack = self.0.get_mut(stack_idx).unwrap();
        stack.0.push_front(crt);
    }
}

struct Stack(VecDeque<Crate>);

impl Stack {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

#[derive(Debug)]
struct Crate(char);
