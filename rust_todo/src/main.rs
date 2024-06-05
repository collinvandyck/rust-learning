use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Template)]
#[template(
    source = r#"
        <h1>Todo List</h1>
        <form action="/add" method="post">
        <input type="text" name="task"/>
        <input type="submit" value="Add Task"/>
        </form>
        <ul>
        {% for task in tasks %}
        <li>{{ task }}</li>
        {% endfor %}
        </ul>
        "#,
    ext = "txt"
)]
struct TodoListTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[derive(Deserialize)]
struct AddTask {
    task: String,
}

static TASKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_tasks))
        .route("/add", post(add_task));
    let listener = std::net::TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    let server = axum::Server::from_tcp(listener).unwrap();
    server.serve(app.into_make_service()).await.unwrap();
}

async fn show_tasks() -> Html<String> {
    let tasks = TASKS.lock().unwrap();
    let tmpl = TodoListTemplate { tasks: &tasks };
    Html(tmpl.render().unwrap())
}

async fn add_task(Form(input): Form<AddTask>) -> impl IntoResponse {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    Redirect::to("/")
}
