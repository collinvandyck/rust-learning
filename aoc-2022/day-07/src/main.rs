use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    process("example.txt");
}

fn process(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        println!("{line}");
    }
}
