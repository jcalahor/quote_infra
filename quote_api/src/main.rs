#![allow(deprecated)]

use axum::response::IntoResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::get,
    Router,
};

use chrono::{FixedOffset, Local, Utc};
use std::sync::Arc;

use log::{error, info};
use quote_lib::{setup_logger, FileNameIdentifiers, RedisHandler, CONFIG};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteRequest {
    quote: String,
    base: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteResponse {
    quote: String,
    base: String,
    date: String,
    rate: f64,
}


pub async fn quote_inquire(
    State(redis_handler): State<Arc<RedisHandler>>,
    Json(payload): Json<QuoteRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    info!("Request: {:?}", payload);
    let utc_now = Utc::now();

    
    let est_offset = FixedOffset::west(5 * 3600); // UTC - 5 hours
    let est_now = utc_now.with_timezone(&est_offset);
    let date = est_now.format("%Y-%m-%d").to_string();

    let quote_envelope_base = match redis_handler
        .get_quote(&date, &payload.base, &"USD".to_string())
        .await
    {
        Ok(Some(quote_envelope)) => Some(quote_envelope),
        Ok(None) => None,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let quote_envelope_quote = match redis_handler
        .get_quote(&date, &payload.quote, &"USD".to_string())
        .await
    {
        Ok(Some(quote_envelope)) => Some(quote_envelope),
        Ok(None) => None,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if quote_envelope_base.is_none() || quote_envelope_quote.is_none() {
        error!("One or both quotes not found.");
        return Err(StatusCode::NOT_FOUND);
    }

    let base_quote = quote_envelope_base.unwrap();
    let quote_quote = quote_envelope_quote.unwrap();

    info!("Found quotes: {:?} - {:?}", base_quote, quote_quote);

    // Create the response based on the quotes fetched
    let response = QuoteResponse {
        quote: quote_quote.base,
        base: base_quote.base,
        date,
        rate: quote_quote.rate / base_quote.rate,
    };

    info!("response with: {:?}", &response);
    Ok(Json(response))
}

pub async fn app() -> Router {
    let redis_handler = match RedisHandler::new(&CONFIG.redis.redis_url).await {
        Ok(handler) => {
            // Successfully created the RedisHandler instance
            Arc::new(handler)
        }
        Err(err) => {
            error!("Error creating Redis handler: {}", err);
            panic!("");
        }
    };
    Router::new()
        .route("/quote", get(quote_inquire))
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(redis_handler)
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
