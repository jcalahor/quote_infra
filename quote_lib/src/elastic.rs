use crate::QuoteEnvelope;
use elasticsearch::{Elasticsearch, IndexParts};
use log::info;
use serde_json::json;
use std::error::Error;
use uuid::Uuid;

pub async fn store_quote_envelope(
    client: &Elasticsearch,
    envelope: &QuoteEnvelope,
) -> Result<(), Box<dyn Error>> {
    let index_id = Uuid::new_v4().to_string();
    let response = client
        .index(IndexParts::IndexId("quotes", &index_id))
        .body(json!({
            "base": envelope.base,
            "quote": envelope.quote,
            "rate": envelope.rate,
            "date": envelope.date,
            "timestamp": envelope.timestamp,
        }))
        .send()
        .await?;

    let response_text = response.text().await?;
    info!("Response: {}", response_text);
    Ok(())
}
