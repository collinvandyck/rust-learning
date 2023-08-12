use std::time::{Duration, Instant};

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

    let (tx, mut rx) = mpsc::channel::<bool>(num_tasks);
    tokio::spawn(generate(sched.clone(), tx));
    let mut num_scheduled = 0;
    let mut num_rejected = 0;
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    interval.tick().await;

    loop {
        tokio::select! {
            _ = interval.tick() => {
                println!("{num_scheduled} {num_rejected}");
                //
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

    /*
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
    */
}

async fn generate(sched: Scheduler, tx: mpsc::Sender<bool>) -> Result<()> {
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
