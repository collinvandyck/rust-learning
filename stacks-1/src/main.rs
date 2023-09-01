#![allow(dead_code)]
use backtrace::Backtrace;
use std::{future::pending, time::Duration};
use tracing_subscriber::FmtSubscriber;

use anyhow::Result;
use rand::{thread_rng, Rng};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::{sleep, Instant},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().finish();
    let _guard = tracing::subscriber::set_default(subscriber);
    let ball = Ball { count: 0 };
    let (tx, mut rx) = mpsc::channel::<Ball>(1);
    let mut txs = vec![];
    for _ in 0..10 {
        let tx = tx.clone();
        let (player_tx, player_rx) = mpsc::channel::<Ball>(1);
        txs.push(player_tx);
        tokio::spawn(async_backtrace::frame!(pending::<()>()));
        tokio::spawn(async move {
            player(tx, player_rx).await;
        });
    }
    tx.send(ball).await?;
    let mut rng = thread_rng();
    let sleep = sleep(Duration::from_millis(100));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            Some(ball) = rx.recv() => {
                info!(ball = ?ball);
                let idx = rng.gen_range(0..txs.len());
                txs[idx].send(ball).await?;
            }
            () = &mut sleep => {
                info!("BOOP");
                sleep.as_mut().reset(Instant::now() + Duration::from_millis(100));
                backtrace();
                return Ok(());
            }
        }
    }
}

async fn async_backtrace() {
    let bt = async_backtrace::taskdump_tree(true);
    println!("{bt}");
}

fn backtrace() {
    let bt = Backtrace::new();
    println!("{bt:?}")
}

async fn dump_stack() {
    backtrace::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();
        println!("{symbol_address:?} {ip:?}");
        backtrace::resolve_frame(frame, |symbol| {
            let sym_name = symbol
                .name()
                .and_then(|n| n.as_str())
                .unwrap_or("unknown_sym");
            let filename = symbol
                .filename()
                .map(|f| f.as_os_str().to_string_lossy().to_string())
                .unwrap_or("unknown_file".to_string());
            let line = symbol
                .lineno()
                .map(|n| format!("{n}"))
                .unwrap_or("".to_string());
            println!("  {sym_name} {filename} {line}");
        });
        println!("");
        true
    })
}

#[async_backtrace::framed]
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
