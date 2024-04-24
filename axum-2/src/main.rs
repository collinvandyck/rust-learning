#![allow(unused)]

use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
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
    let config = Config {
        greeting: "Hello".to_string(),
    };
    let http = Http::start(config).await?;

    let port = http.addr.port();
    let client = async move {
        let url = format!("http://localhost:{port}");
        let ping = reqwest::get(format!("{url}/ping")).await?.text().await?;
        info!("client ping: {ping}");
        let pong = reqwest::get(format!("{url}/pong")).await?.text().await?;
        info!("client pong: {pong}");
        let echo = reqwest::Client::new()
            .post(format!("{url}/echo"))
            .body("hi")
            .send()
            .await?
            .text()
            .await?;
        info!("client echo: {echo}");
        anyhow::Ok(())
    };

    select! {
        res = http.wait() => {
            if let Err(err) = res {
                error!("http failed: {err}");
            } else {
                info!("http quit");
            }
        }
        _ = client => {
            info!("client done");
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

// this config can be populated however we like
#[derive(Clone)]
struct Config {
    greeting: String,
}

/// The App is a configurable struct which sets up the axum router to delegate to its handler
/// methods. The config is used in the handler methods to demonstrate the flow.
struct App {
    router: Option<Router<Arc<State>>>,
    config: Config,
}

#[derive(Serialize, Deserialize)]
struct Echo {
    message: String,
}

#[derive(Clone)]
struct State {
    config: Config,
}

impl State {
    async fn echo(&self, echo: Echo) -> String {
        String::from("echo response")
    }
}

impl App {
    fn new(config: Config) -> Self {
        let mut app = App {
            router: None,
            config: config.clone(),
        };
        let state = State {
            config: config.clone(),
        };
        let state = Arc::new(state);
        let router = Router::new()
            .route("/ping", get(app.ping()))
            .route("/pong", get(app.pong()))
            .route("/echo", post("ok"))
            .with_state(state);
        app.router.replace(router);
        app
    }

    async fn echo(&self, echo: Echo) -> String {
        String::from("echo response")
    }

    fn ping(&self) -> String {
        format!("{} PING", self.config.greeting)
    }

    fn pong(&self) -> String {
        format!("{} PONG", self.config.greeting)
    }

    async fn serve(mut self, listener: TcpListener) -> Result<()> {
        axum::serve(listener, self.router.take().unwrap());
        axum::serve(listener, self.router.take().unwrap())
            .await
            .context("axum quit")?;
        Ok(())
    }
}

impl Http {
    async fn start(config: Config) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
        let addr = listener.local_addr()?;
        let app = App::new(config);
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
