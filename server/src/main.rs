use std::{error::Error, fs::{self, File}, io::{stdin, stdout, Read, Write}, path};

use axum::{http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method}, response::{Html, IntoResponse}, routing, Form, Router};
use chrono::Local;
use rand::seq::SliceRandom;
use tower_http::cors::CorsLayer;

mod structs;
use structs::{RoundSubmit, SubmitArgs, SubmitWithTime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip = "0.0.0.0:80";

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

        let input = s.to_lowercase().replace("\r", "").replace("\n", "");
        let input = input.as_str();

        match input {
            "get" => get_one_submit()?,
            "exit" => break,
            _ => {},
        };

        s = String::new();
    }

    Ok(())
}

fn get_one_submit() -> Result<(), Box<dyn Error>>{
    let pathes = fs::read_dir("./submits/")?;
    let mut v = vec![];
    
    for path in pathes {
        for p in fs::read_dir(path?.path().to_str().unwrap())? {
            let str = p?.path();
            let str = String::from(str.to_str().unwrap());
            
            let mut file = File::open(&str)?;
            let mut s = String::new();
            
            file.read_to_string(&mut s)?;
            
            let submit = serde_json::from_str::<RoundSubmit>(&s)?;
            let range = &submit.rr;

            for _ in -1..range.clone() {
                v.push((str.clone(), submit.clone()));
            }
        }
    }

    v.shuffle(&mut rand::thread_rng());

    match v.pop() {
        Some(o) => {
            let (file, submit) = o;

            println!("Title: {}", submit.submit.title);
            println!("Name: {}", submit.submit.name);
            println!("Description: {}", submit.submit.description);
            println!("RoundTime: {}", submit.rr);

            fs::remove_file(file)?;

            let pathes = fs::read_dir("./submits/")?;

            for path in pathes {
                for p in fs::read_dir(path?.path().to_str().unwrap())? {
                    let str = p?.path();
                    let str = String::from(str.to_str().unwrap());

                    let mut file = File::open(&str)?;
                    let mut s = String::new();
                    
                    file.read_to_string(&mut s)?;
                    
                    let mut submit = serde_json::from_str::<RoundSubmit>(&s)?;
                    submit.rr += 1;
                    
                    let mut file = File::create(&str)?;
                    file.write_all(serde_json::to_string(&submit)?.as_bytes())?;
                }
            }
        },
        None => println!("Submit list is empty"),
    };

    Ok(())
}

fn create_router() -> Router {
    Router::new()
        .route("/api", routing::get(main_handler))
        .route("/api/health", routing::get(health_handler))
        .route("/api/submit", routing::get(submit_handler))
        .route("/api/success/:args", routing::get(success_handler))
        .route("/api/cancel/:args", routing::get(cancel_handler))
        .route("/api/list", routing::get(list_handler))
}

fn error_html() -> Html<String> {
    Html::<String>(include_str!("./static/error.html").to_string())
}

async fn health_handler() -> impl IntoResponse {
    println!("Who check server health");

    let html = format!(
        include_str!("./static/alive.html"),
        Local::now().to_string());

    Html::<String>(html)
}

async fn main_handler() -> impl IntoResponse {
    let html = format!(
        include_str!("./static/index.html"));

    Html::<String>(html)
}

async fn list_handler() -> impl IntoResponse {
    let html = format!(
        include_str!("./static/list.html"),
            match get_table() {
                Ok(o) => o,
                Err(_) => return error_html(),
            });

    Html::<String>(html)
}

fn get_table() -> Result<String, Box<dyn Error>> {
    let pathes = fs::read_dir("./submits/")?;
    let mut result = String::new();
    
    for path in pathes {
        let path = path?;
        let date = format!("{}", &path.file_name().to_str().unwrap());

        for p in fs::read_dir(&path.path().to_str().unwrap())? {
            let str = p?.path();
            let str = String::from(str.to_str().unwrap());
            
            let mut file = File::open(&str)?;
            let mut s = String::new();
            
            file.read_to_string(&mut s)?;
            
            let submit = serde_json::from_str::<RoundSubmit>(&s)?;

            result += format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                submit.submit.title, submit.submit.name, &date, submit.rr).as_str();
        }
    }

    Ok(result)
}

async fn submit_handler() -> impl IntoResponse {
    let html = format!(include_str!("./static/submit.html"));

    Html::<String>(html)
}

async fn success_handler(Form(args): Form<SubmitArgs>)
    -> impl IntoResponse {
    println!("# Add\nTitle: {}\nName: {}\nDescription: {}",
        args.title, args.name, args.description);

    let date: String = Local::now().format("%Y%m%d").to_string();
    let time = Local::now().format("%H%M%S%f.rqj").to_string();

    let html = format!(include_str!("./static/success.html"),
        args.title, args.name, args.description, &date, &time,
        args.title, args.name, args.description, &date, &time);

    if path::Path::new("./submits/").exists() == false {
        match fs::create_dir(path::Path::new("./submits/")) {
            Ok(_) => {},
            Err(_) => return error_html(),
        };
    }
    
    let daily = format!("./submits/{}", &date);
    let daily = daily.as_str();

    if path::Path::new(daily).exists() == false {
        match fs::create_dir(path::Path::new(daily)) {
            Ok(_) => {},
            Err(_) => return error_html(),
        };
    }

    let file = format!("./submits/{}/{}", &date, &time);
    let mut file = match File::create(path::Path::new(&file)) {
        Ok(o) => o,
        Err(_) => return error_html(),
    };

    let args = match serde_json::to_string(&RoundSubmit {
        submit: args,
        rr: 0,
    }) {
        Ok(o) => o,
        Err(_) => return error_html(),
    };

    match file.write(args.as_bytes()) {
        Ok(_) => {},
        Err(_) => return error_html(),
    };

    Html::<String>(html)
}

async fn cancel_handler(Form(args): Form<SubmitWithTime>)
    -> impl IntoResponse {
    println!("# remove\nTitle: {}\nName: {}\nDescription: {}",
        args.title, args.name, args.description);

    let html = format!(include_str!("./static/cancel.html"));

    if path::Path::new("./submits/").exists() == false {
        match fs::create_dir(path::Path::new("./submits/")) {
            Ok(_) => {},
            Err(_) => return error_html(),
        };
    }

    let daily = format!("./submits/{}", &args.date);
    let daily = daily.as_str();

    if path::Path::new(daily).exists() == false {
        match fs::create_dir(path::Path::new(daily)) {
            Ok(_) => {},
            Err(_) => return error_html(),
        };
    }

    let file = format!("./submits/{}/{}", &args.date, &args.time);

    match fs::remove_file(file) {
        Ok(_) => {},
        Err(_) => return error_html(),
    };

    Html::<String>(html)
}