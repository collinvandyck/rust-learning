use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    body::Bytes,
    extract::State,
    routing::{get, post},
    Router,
};
use tokio::sync::{oneshot, Mutex};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let config = Config {
        answer: 42,
        bump: Arc::default(),
    };
    let http = Http::start(config).await?;
    info!("Listening on {}", http.addr);
    let test = async {
        let client = Client(reqwest::Client::new());
        info!("echo ping: {}", client.post_string("/echo", "ping").await?);
        info!("answer: {}", client.get_string("/answer").await?);
        for _ in 0..5 {
            info!("bump: {}", client.get_string("/bump").await?);
        }
        anyhow::Ok(())
    };
    tokio::select! {
        res = http.wait() => {
            if let Err(err) = res {
                error!("axum failed: {err}");
            } else {
                info!("axum quit.");
            }
        }
        res = test => {
            if let Err(err) = res {
                error!("client failed: {err}");
            } else {
                info!("client finished successfully");
            }
        }
    }
    Ok(())
}

/// The config that is passed to Http::start, and ends up getting cloned into the Handler.
#[derive(Clone)]
struct Config {
    answer: i32,
    bump: Arc<Mutex<i32>>,
}

/// Responsible for starting the http server. The wait method can be used to know when it quits.
struct Http {
    addr: SocketAddr,
    rx: oneshot::Receiver<Result<()>>,
}

impl Http {
    async fn start(config: Config) -> Result<Self> {
        type HandlerState = State<Arc<Handler>>;
        let state = Arc::new(Handler::new(config.clone()));
        let app = Router::new()
            .route(
                "/echo",
                post(|State(handler): HandlerState, body: Bytes| async move {
                    handler.echo(body).await
                }),
            )
            .route(
                "/answer",
                get(|State(handler): HandlerState| async move { handler.answer().await }),
            )
            .route(
                "/bump",
                get(|State(handler): HandlerState| async move { handler.bump().await }),
            )
            .with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        let addr = listener.local_addr()?;
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let res = axum::serve(listener, app).await.context("axum quit");
            let _ = tx.send(res);
        });
        Ok(Self { addr, rx })
    }

    async fn wait(self) -> Result<()> {
        self.rx.await?.context("rx fail")?;
        Ok(())
    }
}

struct Handler {
    config: Config,
}

impl Handler {
    fn new(config: Config) -> Self {
        Self { config }
    }

    async fn answer(&self) -> String {
        self.config.answer.to_string()
    }

    async fn echo(&self, bytes: Bytes) -> String {
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    async fn bump(&self) -> String {
        let mut b = self.config.bump.lock().await;
        *b += 1;
        b.to_string()
    }
}

struct Client(reqwest::Client);

impl Client {
    async fn post_string(&self, path: &str, body: &str) -> Result<String> {
        let v = self
            .0
            .post(format!("http://127.0.0.1:3000{path}"))
            .body(body.to_string())
            .send()
            .await?
            .text()
            .await?;
        anyhow::Ok(v)
    }

    async fn get_string(&self, path: &str) -> Result<String> {
        let v = self
            .0
            .get(format!("http://127.0.0.1:3000{path}"))
            .send()
            .await?
            .text()
            .await?;
        anyhow::Ok(v)
    }
}
