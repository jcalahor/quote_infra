use chrono::{DateTime, Utc};
use deadpool_redis::redis::{self, AsyncCommands}; // Import redis from deadpool_redis
use deadpool_redis::{Manager, Pool};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisQuoteError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError), // Use the correct RedisError from deadpool_redis

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),

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
        key: &str,
        payload: String,
        ttl_seconds: i64,
    ) -> Result<bool, RedisQuoteError> {
        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|_| RedisQuoteError::PoolError)?; // Map PoolError to LockingError

        conn.set_ex::<_, _, ()>(key, payload, ttl_seconds as u64)
            .await
            .map_err(RedisQuoteError::RedisError)?;
        Ok(true)
    }
}
