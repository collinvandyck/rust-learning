use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

fn main() {
    let args = Args::parse();
    for line in BufReader::new(File::open(&args.filename).unwrap()).lines() {
        let cube = Cube::parse(line.unwrap());
        println!("{cube:?}");
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: u64,
    y: u64,
    z: u64,
}

impl Cube {
    fn parse(line: String) -> Self {
        let mut iter = line.split(',');
        let x = iter.next().unwrap().parse::<u64>().unwrap();
        let y = iter.next().unwrap().parse::<u64>().unwrap();
        let z = iter.next().unwrap().parse::<u64>().unwrap();
        Self { x, y, z }
    }
    /// Each cube is comprised of 6 squares.
    /// A square is defined by
    ///
    /// (1,1,1) has the following squares,
    ///
    fn squares(&self) {}
}
