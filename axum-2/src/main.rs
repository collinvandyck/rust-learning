#![allow(unused)]

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use std::{io, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::TcpListener,
    select,
    sync::{mpsc, Mutex},
    time::sleep,
};
use tracing::error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("Starting http server...");
    let http = Http::start().await?;

    select! {
        res = http.wait() => {
            if let Err(err) = res {
                error!("http failed: {err}");
            } else {
                info!("http quit");
            }
        }
    }

    Ok(())
}

type IoResultRx = mpsc::Receiver<Result<()>>;
type HttpWait = Arc<tokio::sync::Mutex<IoResultRx>>;

struct Http {
    addr: SocketAddr,
    rx: HttpWait,
}

struct App {
    router: Option<Router<()>>,
}

impl App {
    fn new() -> Self {
        let mut app = App { router: None };
        let router = Router::<()>::new().route("/", get(|| async { "pong" }));
        app.router.replace(router);
        app
    }

    async fn serve(mut self, listener: TcpListener) -> Result<()> {
        axum::serve(listener, self.router.take().unwrap())
            .await
            .context("axum quit")?;
        Ok(())
    }

    fn pong(&self) {}
}

impl Http {
    async fn start() -> Result<Self> {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
        let addr = listener.local_addr()?;
        info!(?addr, "Listening on");
        let app = App::new();
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            let res = app.serve(listener).await;
            tx.send(res).await;
        });
        Ok(Self {
            addr,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    /// returns a future that will produce a result when the axum server quits.
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
