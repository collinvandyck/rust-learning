use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use scheduler_2::scheduler::{Response, Scheduler};
use tokio::sync::mpsc;

#[derive(Parser)]
struct Args {
    #[arg(short, default_value_t = 100000)]
    num_tasks: usize,

    #[arg(short = 't', default_value_t = 1)]
    num_task_types: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let sched = Scheduler::new();
    let num_tasks = args.num_tasks;

    let (tx, mut rx) = mpsc::channel::<bool>(num_tasks);
    tokio::spawn(generate(sched.clone(), tx, args.num_task_types));
    let mut num_scheduled = 0;
    let mut num_rejected = 0;
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    interval.tick().await;
    let mut start = Instant::now();
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let dur = Instant::now().duration_since(start);
                let per_sec = if num_scheduled == 0 {
                    0.0
                } else {
                    num_scheduled as f64 / dur.as_secs_f64()
                };
                let success_pct = num_scheduled as f64 / (num_scheduled as f64 + num_rejected as f64) * 100.0;
                println!("{success_pct:3.0}%\t{per_sec:.0}/sec\t{num_scheduled} {num_rejected}");

                // reset
                num_scheduled=0;
                num_rejected=0;
                start = Instant::now();
            }
            Some(scheduled) = rx.recv() => {
                if scheduled {
                    num_scheduled += 1;
                } else {
                    num_rejected += 1;
                }
            }
        }
    }
}

async fn generate(sched: Scheduler, tx: mpsc::Sender<bool>, num_types: usize) -> Result<()> {
    loop {
        let tx = tx.clone();
        let tx2 = tx.clone();
        let res = sched
            .schedule("task1", async move {
                tx.send(true).await.unwrap();
            })
            .await?;
        if res == Response::Rejected {
            tx2.send(false).await.unwrap();
        }
    }
}
