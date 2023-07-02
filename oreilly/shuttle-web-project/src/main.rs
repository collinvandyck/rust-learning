use axum::{response::IntoResponse, routing, Router};

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/", routing::get(hello_world));
    Ok(app.into())
}
