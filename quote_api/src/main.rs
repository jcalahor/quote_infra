use axum::response::IntoResponse;
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    routing::get,
    Router,
};
use chrono::Local;

use log::info;
use quote_lib::{setup_logger, FileNameIdentifiers, QuoteEnvelope, RedisHandler, CONFIG};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QuoteRequest {
    quote: String,
    base: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QuoteResponse {
    quote: String,
    base: String,
    date: String,
}

async fn quote_inquire(Json(payload): Json<QuoteRequest>) -> Result<impl IntoResponse, StatusCode> {
    let response = QuoteResponse {
        quote: "".to_string(),
        base: "".to_string(),
        date: "".to_string(),
    };
    info!("response with: {:?}", &response);
    Ok(Json(response))
}

pub async fn app() -> Router {
    Router::new()
        .route("/quote", get(quote_inquire))
        .route("/", get(|| async { "Hello, World!" }))
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=1000);
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let fni = FileNameIdentifiers {
        time_stamp: timestamp,
        random_nbr: random_number,
        name_suffix: "quote-api-service-output.log".to_string(),
    };
    setup_logger(&fni).expect("Failed to set up logger");

    let address = format!("{}:{}", CONFIG.server.address, CONFIG.server.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Server started at {}", &address);
    axum::serve(listener, app().await.into_make_service())
        .await
        .unwrap();
}
