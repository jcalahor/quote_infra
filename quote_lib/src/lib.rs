use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteEnvelope {
    date: String,
    rate: f64,
    quote: String,
    base: String,
    timestamp: u64,
}

impl QuoteEnvelope {
    pub fn new(date: String, rate: f64, quote: String, base: String, timestamp: u64) -> Self {
        QuoteEnvelope {
            date,
            rate,
            quote,
            base,
            timestamp,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

mod redis_handler;
pub use redis_handler::RedisHandler;
