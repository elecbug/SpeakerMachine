use std::{error::Error, fs::{self, File}, io::{stdin, stdout, Write}, path};

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::{Html, IntoResponse}, routing, Form, Router};
use chrono::Local;
use tower_http::cors::CorsLayer;

mod structs;
use structs::{SubmitArgs, RoundSubmit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip = "localhost:80";

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
    
    let mut s = String::new();

    loop {
        print!(">> ");
        
        stdout().flush()?;
        stdin().read_line(&mut s)?;
        
        match s.to_lowercase().as_str() {
            "get" => {
                get_one_submit();
            },
            _ => {},
        }
    }
}

fn get_one_submit() {
}

fn create_router() -> Router {
    Router::new()
        .route("/api/health", routing::get(health_handler))
        .route("/api/submit", routing::get(submit_screen_handler))
        .route("/api/submit/:args", routing::get(submit_handler))
}

async fn health_handler() -> impl IntoResponse {
    println!("Who check server health");

    let html = format!(include_str!("static/alive.html"), Local::now().to_string());

    Html::<String>(html)
}

async fn submit_screen_handler() -> impl IntoResponse {
    let html = format!(include_str!("static/submit.html"));

    Html::<String>(html)
}

async fn submit_handler(Form(args): Form<SubmitArgs>) -> impl IntoResponse {
    println!("Title: {}\nName: {}\nDescription: {}", args.title, args.name, args.description);

    let html = format!(include_str!("static/success.html"), args.title, args.name, args.description);

    if path::Path::new("./submits/").exists() == false {
        match fs::create_dir(path::Path::new("./submits/")) {
            Ok(_) => {},
            Err(_) => return Html::<String>("Error".to_string()),
        };
    }

    let file = Local::now().format("%Y%m%d%H%M%S%f.rqj").to_string();
    let file = format!("./submits/{}", file);
    let mut file = match File::create(path::Path::new(&file)) {
        Ok(o) => o,
        Err(_) => return Html::<String>("Error".to_string()),
    };

    let args = match serde_json::to_string(&RoundSubmit {submit: args, rr: 0}) {
        Ok(o) => o,
        Err(_) => return Html::<String>("Error".to_string()),
    };

    match file.write(args.as_bytes()) {
        Ok(_) => {},
        Err(_) => return Html::<String>("Error".to_string()),
    };

    Html::<String>(html)
}