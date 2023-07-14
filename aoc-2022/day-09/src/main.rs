use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        println!("{line}");
    }
}
