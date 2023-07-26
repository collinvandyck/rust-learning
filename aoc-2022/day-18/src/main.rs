use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

fn main() {
    let args = Args::parse();
    for line in BufReader::new(File::open(&args.filename).unwrap()).lines() {
        let point = Point::parse(line.unwrap());
        println!("{point:?}");
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn parse(line: String) -> Self {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<u64>().unwrap();
        let y = iter.next().unwrap().parse::<u64>().unwrap();
        let z = iter.next().unwrap().parse::<u64>().unwrap();
        Self { x, y, z }
    }
}
