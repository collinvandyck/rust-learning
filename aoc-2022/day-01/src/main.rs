use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<u32, io::Error> {
    let file = File::open("input.txt")?;
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line?;
        println!("{line}");
    }
    Ok(0)
}
