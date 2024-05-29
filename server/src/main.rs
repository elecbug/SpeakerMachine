use std::error::Error;

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::IntoResponse, routing, Json, Router};
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
    const MESSAGE: &str = "Alive The Channel";
    println!("CHECK-HEALTH");

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}