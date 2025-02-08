use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub topic: String,
    pub channel_size: usize,
}

#[derive(Deserialize, Debug)]
pub struct RedisConfig {
    pub redis_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Config {
            server: ServerConfig {
                address: env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"),
                port: env::var("SERVER_PORT")
                    .expect("SERVER_PORT must be set")
                    .parse()
                    .expect("SERVER_PORT must be a valid u16"),
            },
            kafka: KafkaConfig {
                bootstrap_servers: env::var("KAFKA_BOOTSTRAP_SERVERS")
                    .expect("KAFKA_BOOTSTRAP_SERVERS must be set"),
                group_id: env::var("KAFKA_GROUP_ID").expect("KAFKA_GROUP_ID must be set"),
                topic: env::var("KAFKA_TOPIC").expect("KAFKA_TOPIC must be set"),
                channel_size: env::var("KAFKA_CHANNEL_SIZE")
                    .expect("KAFKA_CHANNEL_SIZE must be set")
                    .parse()
                    .expect("KAFKA_CHANNEL_SIZE must be a valid usize"),
            },
            redis: RedisConfig {
                redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            },
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::from_env);
