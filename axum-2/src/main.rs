#![allow(unused)]

use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use axum::{routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let http = Http::start().await?;
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

struct Http {
    addr: SocketAddr,
    rx: tokio::sync::mpsc::Receiver<std::result::Result<(), std::io::Error>>,
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
        Ok(Self { addr, rx })
    }
}
