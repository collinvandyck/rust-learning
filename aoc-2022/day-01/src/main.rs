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
    elves.sort_by_key(|e| -1 * e.calories as i32);
    let most = elves.iter().next().unwrap();
    println!(
        "Most calories carries by elf {}: {}",
        most.id, most.calories
    );
    let top3: u32 = elves.iter().take(3).map(|e| e.calories).sum();
    println!("Top 3 calories: {top3}");
}

#[derive(Debug, Clone)]
struct Elf {
    id: u32,
    calories: u32,
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
                calories: count,
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
            calories: count,
        })
    }
    Ok(elves)
}
