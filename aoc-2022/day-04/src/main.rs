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
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        dbg!((first, second));
    }
}
