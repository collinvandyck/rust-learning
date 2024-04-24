use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    body::Bytes,
    extract::State,
    routing::{get, post},
    Router,
};
use tokio::sync::oneshot;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let config = Config { answer: 42 };
    let http = Http::start(config).await?;
    info!("Listening on {}", http.addr);
    let test = async {
        let client = Client(reqwest::Client::new());
        info!("root: {}", client.get_string("/root").await?);
        info!("echo ping: {}", client.post_string("/echo", "ping").await?);
        info!("answer: {}", client.get_string("/answer").await?);
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

#[derive(Clone)]
struct Config {
    answer: i32,
}

struct Http {
    addr: SocketAddr,
    rx: oneshot::Receiver<Result<()>>,
}

struct Handler {
    config: Config,
}

impl Handler {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Http {
    async fn start(config: Config) -> Result<Self> {
        let state = Arc::new(Handler::new(config.clone()));
        let app = Router::new()
            .route("/root", get(root))
            .route("/echo", post(echo))
            .route("/answer", get(answer))
            .route("/", get(|| async { "Hello, World!" }))
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

async fn echo(body: Bytes) -> String {
    String::from_utf8(body.to_vec()).unwrap()
}

async fn root() -> &'static str {
    "hi"
}

async fn answer(State(handler): State<Arc<Handler>>) -> String {
    handler.config.answer.to_string()
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
