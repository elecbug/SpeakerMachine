use std::{error::Error, fmt::format};

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::{Html, IntoResponse}, routing, Router};
use chrono::Local;
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip = "210.125.31.240:80";

    let cors = CorsLayer::new()
        .allow_origin(ip.parse::<HeaderValue>()?)
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);
    let listener = tokio::net::TcpListener::bind(ip).await?;

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    loop {

    }
}

fn create_router() -> Router {
    Router::new()
        .route("/api/health", routing::get(health_handler))
}

async fn health_handler() -> impl IntoResponse {
    println!("CHECK-HEALTH");

    let html = format!(include_str!("static/alive.html"), Local::now().to_string());

    Html::<String>(html)
}