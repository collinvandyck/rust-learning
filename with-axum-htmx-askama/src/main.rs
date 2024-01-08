use anyhow::Context;
use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing router");
    let router = Router::new().route("/", get(hello));
    let port = 8000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("Router initialized");

    info!("Starting server");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("bind to port")?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("serve failed")?;
    info!("Server quit");
    Ok(())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate;
    HtmlTemplate(template);
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

struct HtmlTemplate<T>(T);
