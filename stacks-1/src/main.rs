#![allow(dead_code)]
use std::time::Duration;

use anyhow::Result;
use rand::{thread_rng, Rng};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::sleep,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ball = Ball { count: 0 };
    let (tx, mut rx) = mpsc::channel::<Ball>(1);
    let mut txs = vec![];
    for _ in 0..10 {
        let tx = tx.clone();
        let (player_tx, player_rx) = mpsc::channel::<Ball>(1);
        txs.push(player_tx);
        tokio::spawn(async move {
            player(tx, player_rx).await;
        });
    }
    tx.send(ball).await?;
    let mut rng = thread_rng();
    while let Some(ball) = rx.recv().await {
        println!("ball: #{ball:?}");
        let idx = rng.gen_range(0..txs.len());
        txs[idx].send(ball).await?;
    }
    Ok(())
}

async fn player(tx: Sender<Ball>, mut rx: Receiver<Ball>) {
    while let Some(mut ball) = rx.recv().await {
        ball.count += 1;
        sleep(Duration::from_millis(100)).await;
        if tx.send(ball).await.is_err() {
            break;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ball {
    count: u64,
}
