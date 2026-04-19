use std::env;
use std::fs;
use std::process;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::any,
    Router,
};
use serde_json::Value;

type AppState = Arc<serde_json::Map<String, Value>>;

async fn route_handler(State(map): State<AppState>, Path(key): Path<String>) -> Response {
    match map.get(&key) {
        Some(v) => (
            [("content-type", "application/json")],
            v.to_string(),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "Not Found").into_response(),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(f) => f,
        None => {
            eprintln!("usage: rumose <routes.json>");
            process::exit(1);
        }
    };

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error reading '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let v: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error parsing '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let map = match v {
        Value::Object(map) => map,
        _ => {
            eprintln!("error: '{}' must contain a JSON object at the top level", filename);
            process::exit(1);
        }
    };

    let state: AppState = Arc::new(map);

    let app = Router::new()
        .route("/*key", any(route_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1337").await.unwrap();
    println!("listening on 127.0.0.1:1337");
    axum::serve(listener, app).await.unwrap();
}
