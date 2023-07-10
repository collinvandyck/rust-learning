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
        let chars = &mut line.chars();
        loop {
            let part: String = chars.take(3).collect();
            if part.len() < 3 {
                println!("Done");
                break;
            }
            dbg!(part);
            let _ = chars.take(1).collect::<Vec<_>>();
        }
    }
}

#[allow(dead_code)]
struct Ship(Vec<Stack>);

struct Stack(Vec<Crate>);

struct Crate(char);
