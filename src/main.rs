mod prompts;

use core::fmt;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
    Json, Router,
};
use inquire::{Select, Text};
use serde_json::Value;
use std::{fs, str::FromStr, thread::sleep, time::Duration};
use strum::VariantNames;
use strum_macros::{EnumString, VariantNames};

#[derive(Debug, EnumString, VariantNames)]
enum HttpMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl fmt::Display for HttpMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() {
    let path = Text::new("What is the path of your route?")
        .prompt()
        .unwrap();
    let http_method = Select::new(
        "What HTTP method should listen to?",
        HttpMethods::VARIANTS.to_vec(),
    )
    .prompt()
    .unwrap();
    let http_response_status = Select::new(
        "What should be the response HTTP status?",
        (100..599).map(|n| n.to_string()).collect(),
    )
    .prompt()
    .unwrap();
    let http_response =
        Text::new("What should be the response? (Write a file path or leave empty for none)")
            .with_autocomplete(prompts::file_completion::FilePathCompleter::default())
            .prompt()
            .unwrap();
    let delay = Text::new("There's any delay that you want on the route? (in seconds)")
        .prompt()
        .unwrap();

    let route_handler = || {
        let handle = move || {
            handler(
                http_response,
                http_response_status.parse::<u16>().unwrap(),
                if delay.len() > 0 {
                    Some(delay.parse::<usize>().unwrap())
                } else {
                    None
                },
            )
        };

        match HttpMethods::from_str(http_method).unwrap() {
            HttpMethods::GET => get(handle),
            HttpMethods::POST => post(handle),
            HttpMethods::PUT => put(handle),
            HttpMethods::DELETE => delete(handle),
            HttpMethods::PATCH => patch(handle),
        }
    };

    let app = Router::new().route(path.as_str(), route_handler());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999")
        .await
        .unwrap();

    println!("Please test your route with: curl -v -X {http_method} http://localhost:9999{path}");

    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    response_path: String,
    response_status: u16,
    delay: Option<usize>,
) -> impl IntoResponse {
    if let Some(d) = delay {
        sleep(Duration::from_secs(d.try_into().unwrap()));
    }

    let response = if response_path.len() > 0 {
        serde_json::from_str::<Value>(&fs::read_to_string(response_path).unwrap()).unwrap()
    } else {
        serde_json::Value::String("".to_string())
    };

    (
        StatusCode::from_u16(response_status).unwrap(),
        Json(response),
    )
}
