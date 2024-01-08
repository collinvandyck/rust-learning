use anyhow::Context;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tower_http::services::ServeDir;
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
    let assets_path = std::env::current_dir().unwrap();
    let api = Router::new().route("/hello", get(hello_from_the_server));
    let router = Router::new()
        .nest("/api", api)
        .route("/", get(hello))
        .route("/another-page", get(another_page))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    let port = 8000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    info!("Router initialized");

    info!("Starting server");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("bind to port")?;
    let addr = listener.local_addr().context("get listener addr")?;
    info!("Listening on {}", addr);
    axum::serve(listener, router.into_make_service())
        .await
        .context("serve failed")?;
    info!("Server quit");

    Ok(())
}

async fn hello_from_the_server() -> impl IntoResponse {
    "Hello!"
}

#[derive(Template)]
#[template(path = "another_page.html")]
struct AnotherPageTemplate;

async fn another_page() -> impl IntoResponse {
    let template = AnotherPageTemplate;
    let tmpl = HtmlTemplate(template);
    tmpl
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate;
    let tmpl = HtmlTemplate(template);
    tmpl
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => {
                let code = StatusCode::INTERNAL_SERVER_ERROR;
                let msg = format!("Failed to render template: {err}");
                (code, msg).into_response()
            }
        }
    }
}
