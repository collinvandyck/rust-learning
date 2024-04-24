#![allow(unused)]

use std::{io, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use tokio::sync::{mpsc, Mutex};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let http = Http::start().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

type IoResult = std::result::Result<(), io::Error>;
type IoResultRx = mpsc::Receiver<IoResult>;
type HttpWait = Arc<tokio::sync::Mutex<IoResultRx>>;

struct Http {
    addr: SocketAddr,
    rx: HttpWait,
}

struct Ctrl {}

impl Http {
    async fn start() -> Result<Self> {
        let app = Router::new().route("/", get(|| async { "pong" }));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
        let addr = listener.local_addr()?;
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        info!(?addr, "Listening on");
        tokio::spawn(async move {
            let res = axum::serve(listener, app).await;
            tx.send(res).await;
        });
        Ok(Self {
            addr,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    async fn wait(&self) -> Result<()> {
        self.rx
            .lock()
            .await
            .recv()
            .await
            .context("http listener quit")?;
        Ok(())
    }
}
