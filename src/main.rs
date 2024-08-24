mod app_requirements;
mod prompts;

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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let requirements = ApplicationRequirements::get_from_user(args);

    let route_handler = || {
        let app = requirements.clone();
        let handle = || handler(app);

        match requirements.http_method {
            HttpMethods::GET => get(handle),
            HttpMethods::POST => post(handle),
            HttpMethods::PUT => put(handle),
            HttpMethods::DELETE => delete(handle),
            HttpMethods::PATCH => patch(handle),
        }
    };

    let router = Router::new().route(&requirements.path.as_str(), route_handler());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    println!(
        "Please test your route with: curl -v -X {} http://localhost:9999{}",
        &requirements.http_method, &requirements.path
    );

    axum::serve(listener, router).await.unwrap();
}

async fn handler(app: ApplicationRequirements) -> impl IntoResponse {
    app.try_sleep();

    let status = StatusCode::from_u16(app.http_response_status).unwrap();

    match app.http_response_path {
        Some(path) => Response::builder()
            .status(status)
            .body(Body::from(fs::read_to_string(path).unwrap()))
            .unwrap(),
        None => Response::builder()
            .status(status)
            .body(Body::empty())
            .unwrap(),
    }
}
