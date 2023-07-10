use std::{
    fs::File,
    io::{BufRead, BufReader},
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
        loop {
            let part: String = chars.by_ref().take(3).collect();
            if part.len() < 3 {
                // end of line
                println!("Done");
                break;
            }
            dbg!(part);
            // discard the space
            chars.next();
        }
    }
}

#[allow(dead_code)]
struct Ship(Vec<Stack>);

struct Stack(Vec<Crate>);

struct Crate(char);
