use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("input.txt");
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
    // process the moves.
    for line in iter {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        let op = MoveOp::from(&line);
        println!("Op: {op:?}");
        ship.perform(op);
        println!("Ship:\n{ship}");
    }
    println!("Tops:");
    ship.print_tops();
}

#[derive(Debug)]
struct MoveOp {
    count: usize,
    from: usize,
    to: usize,
}

impl MoveOp {
    fn from(s: &str) -> MoveOp {
        let mut iter = s.split(" ");
        iter.next(); // move
        let count = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // from
        let from = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // to
        let to = iter.next().unwrap().parse::<usize>().unwrap();
        MoveOp { count, from, to }
    }
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
    fn perform(&mut self, mv: MoveOp) {
        let mut crts = vec![];
        for _ in 0..mv.count {
            let from_idx = mv.from - 1;
            let crt = self.pop_from(from_idx);
            crts.push(crt);
        }
        crts.reverse();
        let to_idx = mv.to - 1;
        for crt in crts {
            self.push_to(to_idx, crt);
        }
    }
    fn pop_from(&mut self, stack_idx: usize) -> Crate {
        self.0.get_mut(stack_idx).unwrap().0.pop_back().unwrap()
    }
    fn push_to(&mut self, stack_idx: usize, crt: Crate) {
        self.0.get_mut(stack_idx).unwrap().0.push_back(crt)
    }
    fn print_tops(&self) {
        self.0.iter().for_each(|s| {
            let f = s.0.get(s.0.len() - 1).unwrap();
            print!("{}", f.0);
        });
        println!();
    }
}

impl Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        let max_stack_len = self.0.iter().map(|s| s.0.len()).max().unwrap_or(0);
        for ms in (0..max_stack_len).rev() {
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
    fn pop(&mut self) -> Crate {
        self.0.pop_back().unwrap()
    }
}

#[derive(Debug)]
struct Crate(char);
