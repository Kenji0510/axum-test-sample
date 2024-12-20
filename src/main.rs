// src/main.rs
use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello_handler() -> Json<HelloResponse> {
    println!("--> {:12} - Accessed /hello", "HANDLER");
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
    })
}

pub fn routes() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

#[tokio::main]
async fn main() {
    let app = routes();

    println!("--> {:12} - Started running server on port 8080", "INFO");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
