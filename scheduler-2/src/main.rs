use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use scheduler_2::scheduler::{Response, Scheduler};
use tokio::sync::mpsc;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value_t = 100000)]
    num_tasks: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();
    let args = Args::parse();
    let sched = Scheduler::new();
    let num_tasks = args.num_tasks;
    let mut num_scheduled = 0;
    let (tx, mut rx) = mpsc::channel::<()>(num_tasks);
    for _ in 0..num_tasks {
        let tx = tx.clone();
        let res = sched
            .schedule("task1", async move {
                tx.send(()).await.unwrap();
            })
            .await?;
        if res == Response::Scheduled {
            num_scheduled += 1;
        }
    }
    for _ in 0..num_scheduled {
        rx.recv().await.unwrap();
    }
    let dur = Instant::now().duration_since(start);
    let per_sec = num_scheduled as f64 / dur.as_secs_f64();
    println!(
        "{per_sec:.0}/sec. scheduled:{num_scheduled} rejected : {} dur: {dur:?}",
        num_tasks - num_scheduled
    );
    Ok(())
}
