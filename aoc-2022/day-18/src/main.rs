use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

fn main() {
    let args = Args::parse();
    for line in BufReader::new(File::open(&args.filename).unwrap()).lines() {
        let line = line.unwrap();
        println!("{line}");
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}
