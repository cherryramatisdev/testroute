mod app_requirements;
mod prompts;
mod tokenizer;

use app_requirements::{ApplicationRequirements, HttpMethods};
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Router,
};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,

    #[arg(short, long)]
    method: Option<HttpMethods>,

    #[arg(short, long)]
    status: Option<u16>,

    #[arg(short, long)]
    response: Option<String>,

    #[arg(short, long)]
    delay: Option<String>,

    #[arg(short, long)]
    import: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let requirements = if let Some(import) = args.import {
        let file = fs::read_to_string(import).unwrap();
        let tokens = tokenizer::tokens::parse(file);
        tokenizer::ast::parse_requirements(&tokens).unwrap()
    } else {
        vec![ApplicationRequirements::get_from_user(args)]
    };

    let mut router = Router::new();

    for requirement in &requirements {
        let route_handler = || {
            let app = requirement.clone();
            let handle = || handler(app);

            match requirement.http_method {
                HttpMethods::GET => get(handle),
                HttpMethods::POST => post(handle),
                HttpMethods::PUT => put(handle),
                HttpMethods::DELETE => delete(handle),
                HttpMethods::PATCH => patch(handle),
            }
        };

        router = router.route(&requirement.path, route_handler());
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    println!("Server is running on http://localhost:9999");
    println!("Available routes:");
    for requirement in &requirements {
        println!(
            "  {} {} - Status: {}",
            requirement.http_method, requirement.path, requirement.http_response_status
        );
    }

    axum::serve(listener, router).await.unwrap();
}

async fn handler(app: ApplicationRequirements) -> impl IntoResponse {
    app.try_sleep();

    let status = StatusCode::from_u16(app.http_response_status).unwrap();

    match (app.http_response_path, app.http_response_body) {
        (Some(path), None) => Response::builder()
            .status(status)
            .body(Body::from(fs::read_to_string(path).unwrap()))
            .unwrap(),
        (None, Some(body)) => Response::builder()
            .status(status)
            .body(Body::from(body))
            .unwrap(),
        _ => Response::builder()
            .status(status)
            .body(Body::empty())
            .unwrap(),
    }
}