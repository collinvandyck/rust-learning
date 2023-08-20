use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use anyhow::anyhow;
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use error::AppError;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let state = SharedState::default();
    let router = Router::new()
        .route("/", get(hello))
        .route("/user", get(user))
        .route("/age", post(age))
        .route("/error", get(gen_error))
        .route("/tryerror", get(try_error))
        .route("/state", get(get_state))
        .route("/kv/:key", get(get_key))
        .route("/kv/:key", post(set_key))
        .with_state(state.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Serialize, Clone)]
struct AppState {
    kvs: HashMap<String, i32>,
}

async fn set_key(
    Path(key): Path<String>,
    State(state): State<SharedState>,
    bytes: Bytes,
) -> Result<String, StatusCode> {
    let mut state = state.write().unwrap();
    let l: i32 = bytes.len() as i32;
    state.kvs.insert(key, l);
    Ok(format!("{l}"))
}

async fn get_key(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<String, StatusCode> {
    let state = state.read().unwrap();
    let val = state.kvs.get(&key);
    val.map(|v| format!("{v}"))
        .ok_or_else(|| StatusCode::NOT_FOUND)
}

async fn get_state(State(state): State<SharedState>) -> Result<impl IntoResponse, AppError> {
    let state = state.read().unwrap().clone();
    Ok(Json(state))
}

async fn try_error() -> Result<impl IntoResponse, AppError> {
    let op = || -> Result<String, AppError> {
        use rand::Rng;
        let mut rng = thread_rng();
        let n = rng.gen::<f64>();
        if n < 0.5 {
            Err(anyhow!("ERR: {n}\n").into())
        } else {
            Ok(format!("OK:  {n}\n"))
        }
    };
    op()
}

async fn gen_error() -> Result<impl IntoResponse, AppError> {
    use rand::Rng;
    let mut rng = thread_rng();
    let n = rng.gen::<f64>();
    if n < 0.5 {
        Err(anyhow!("{n}").into())
    } else {
        Ok(format!("{n}"))
    }
}

async fn age(Json(mut payload): Json<User>) -> impl IntoResponse {
    payload.age += 1;
    (StatusCode::OK, Json(payload))
}

async fn user() -> (StatusCode, Json<User>) {
    let user = User {
        name: "Collin".to_string(),
        age: 48,
    };
    (StatusCode::OK, Json(user))
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}
