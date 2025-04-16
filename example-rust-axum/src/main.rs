use axum::{Router};
use axum::routing::{get};
use axum::response::{IntoResponse};
use axum::http::StatusCode;
use std::error::Error;

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is healthy")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Could not initialize TcpListener");

    axum::serve(listener, app)
        .await
        .expect("Could not successfully create server");

    Ok(())
}