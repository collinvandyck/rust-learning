use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');
        let first = Range::from_str(parts.next().unwrap());
        let second = Range::from_str(parts.next().unwrap());
        dbg!((first, second));
    }
}

#[derive(Debug)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split('-');
        let from = iter.next().unwrap().parse::<i32>().unwrap();
        let to = iter.next().unwrap().parse::<i32>().unwrap();
        Self { from, to }
    }
}
