use crate::quote_envelope::QuoteEnvelope;
use deadpool_redis::redis::{self, AsyncCommands}; // Import redis from deadpool_redis
use deadpool_redis::{Manager, Pool};
use serde_json::Error as SerdeError;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum RedisQuoteError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError), // Use the correct RedisError from deadpool_redis

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),

    #[error("DeSerializationError error")]
    DeserializationError,

    #[error("Pool error")]
    PoolError,
}

#[derive(Clone)]
pub struct RedisHandler {
    redis_pool: Pool,
}

impl RedisHandler {
    pub async fn new(redis_url: &str) -> Result<Self, RedisQuoteError> {
        let manager = Manager::new(redis_url).map_err(RedisQuoteError::RedisError)?;

        let pool = Pool::builder(manager)
            .max_size(16)
            .build()
            .map_err(|_| RedisQuoteError::PoolError)?; // Map any pool error to LockingError::PoolError

        Ok(Self { redis_pool: pool })
    }

    pub async fn store_quote(
        &self,
        quote_envelope: &QuoteEnvelope,
        ttl_seconds: i64,
    ) -> Result<bool, RedisQuoteError> {
        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|_| RedisQuoteError::PoolError)?;

        let key = format!(
            "{}_{}_{}",
            quote_envelope.date, quote_envelope.base, quote_envelope.quote
        );
        conn.set_ex::<_, _, ()>(key, quote_envelope.to_json(), ttl_seconds as u64)
            .await
            .map_err(RedisQuoteError::RedisError)?;
        Ok(true)
    }

    pub async fn get_quote(
        &self,
        date: &String,
        base: &String,
        quote: &String,
    ) -> Result<Option<QuoteEnvelope>, RedisQuoteError> {
        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|_| RedisQuoteError::PoolError)?;

        let key = format!("{}_{}_{}", date, base, quote);

        match conn.get::<_, Option<String>>(key).await {
            Ok(Some(json_str)) => QuoteEnvelope::from_json(&json_str)
                .map(Some)
                .map_err(|_| RedisQuoteError::DeserializationError),
            Ok(None) => Ok(None),
            Err(e) => Err(RedisQuoteError::RedisError(e)),
        }
    }
}
