use clap::Parser;
use rand::{thread_rng, Rng};

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 1000000)]
    runs: usize,
}

fn main() {
    let args = Args::parse();
    let first_choice = run_sim(args.runs, false);
    let second_choice = run_sim(args.runs, true);
    println!("1st choice wins: {first_choice:0.2}%");
    println!("2nd choice wins: {second_choice:0.2}%");
}

fn run_sim(runs: usize, second_choice: bool) -> f64 {
    let mut rng = thread_rng();
    let mut wins = 0;
    for _ in 0..runs {
        let mut doors = [false; 3];
        doors[rng.gen_range(0..3)] = true;
        let choice = rng.gen_range(0..3);
        if second_choice {
            // reveal one of the false doors and choose the other one
            let bad: usize = doors
                .iter()
                .enumerate()
                .find(|(idx, val)| idx != &choice && !**val)
                .map(|(idx, _)| idx)
                .unwrap();
            let other = (0..doors.len())
                .find(|idx| idx != &bad && idx != &choice)
                .unwrap();
            if doors[other] {
                wins += 1;
                continue;
            }
        } else {
            if doors[choice] {
                wins += 1;
                continue;
            }
        }
    }
    (wins as f64) / (runs as f64) * 100.0
}
