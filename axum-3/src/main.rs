use anyhow::Result;
use axum::{
    body::Bytes,
    routing::{get, post},
    Router,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    tracing::info!("Hello");

    // build our application with a single route
    let app = Router::new()
        .route("/root", get(root))
        .route("/echo", post(echo))
        .route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = Client(reqwest::Client::new());
    info!("root: {}", client.get_string("/root").await?);
    info!("echo ping: {}", client.post_string("/echo", "ping").await?);
    Ok(())
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

async fn echo(body: Bytes) -> String {
    String::from_utf8(body.to_vec()).unwrap()
}

async fn root() -> &'static str {
    "hi"
}
