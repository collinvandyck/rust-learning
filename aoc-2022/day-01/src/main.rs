use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

fn main() {
    let mut elves = match run() {
        Ok(elves) => elves,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    elves = dbg!(elves);
    elves.sort_by_key(|e| -1 * e.presents as i32);
    let most = elves.iter().next().unwrap();
    dbg!(most);
}

#[derive(Debug, Clone)]
struct Elf {
    id: u32,
    presents: u32,
}

fn run() -> Result<Vec<Elf>, io::Error> {
    let file = File::open("input.txt")?;
    let read = BufReader::new(file);
    let mut elves = vec![];
    let mut elf_id = 1 as u32;
    let mut count = 0 as u32;
    for line in read.lines() {
        let line = line?;
        if line == "" {
            elves.push(Elf {
                id: elf_id,
                presents: count,
            });
            elf_id += 1;
            count = 0;
            continue;
        }
        let delta: u32 = line.parse().unwrap();
        count += delta;
    }
    if count != 0 {
        elves.push(Elf {
            id: elf_id,
            presents: count,
        })
    }
    Ok(elves)
}
