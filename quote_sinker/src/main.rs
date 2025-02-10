use elasticsearch::{http::transport::Transport, Elasticsearch, Error};
use std::panic::panic_any;

use chrono::Local;
use fast_log::Config as FastLocaConfig;
use log::{error, info};
use quote_lib::{
    setup_logger, store_quote_envelope, FileNameIdentifiers, QuoteEnvelope, RedisHandler, CONFIG,
};
use rand::Rng;
use std::error::Error as StdError;

use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message as RdMesssage,
};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

const EXPIRE_SECONDS: i64 = 3600 * 24 * 7;

fn create_kafka_consumer() -> BaseConsumer {
    match ClientConfig::new()
        .set("bootstrap.servers", &CONFIG.kafka.bootstrap_servers) // Replace with actual config
        .set("group.id", &CONFIG.kafka.group_id) // Replace with actual group ID
        .create::<BaseConsumer>()
    {
        Ok(consumer) => {
            info!("Kafka consumer successfully created!");
            consumer
        }
        Err(err) => {
            error!("Failed to create Kafka consumer: {}", err);
            panic!("Kafka consumer creation failed");
        }
    }
}

fn connect_to_elasticsearch() -> Result<Elasticsearch, Box<dyn StdError>> {
    let transport = Transport::single_node(&CONFIG.elasticsearch.host)?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=1000);
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let fni = FileNameIdentifiers {
        time_stamp: timestamp,
        random_nbr: random_number,
        name_suffix: "quote-sinker-output.log".to_string(),
    };
    setup_logger(&fni).expect("Failed to set up logger");

    let client = connect_to_elasticsearch().unwrap();
    let consumer = create_kafka_consumer();
    consumer
        .subscribe(&[&CONFIG.kafka.topic])
        .expect("topic subscribe failed");

    info!("Starting background process...");

    let redis_handler = match RedisHandler::new(&CONFIG.redis.redis_url).await {
        Ok(handler) => {
            // Successfully created the RedisHandler instance
            handler
        }
        Err(err) => {
            error!("Error creating Redis handler: {}", err);
            panic!("");
        }
    };

    loop {
        match consumer.poll(Duration::from_secs(1)) {
            Some(Ok(msg)) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    match payload {
                        Ok(json) => match QuoteEnvelope::from_json(json) {
                            Ok(quote) => {
                                info!("Received Quote: {:?}", quote);
                                match redis_handler.store_quote(&quote, EXPIRE_SECONDS).await {
                                    Ok(_) => {
                                        info!("Stored Quote: {:?}", quote);
                                    }
                                    Err(err) => error!("{}", err),
                                }
                                match store_quote_envelope(&client, &quote).await {
                                    Ok(_) => {
                                        info!("Quote successfully stored in ES{:?}", quote);
                                    }
                                    Err(e) => {
                                        error!("Failed to store quote envelope: {}", e);
                                    }
                                }
                            }
                            Err(err) => {
                                error!("Failed to parse message: {}", err);
                            }
                        },
                        Err(err) => {
                            error!("Invalid UTF-8 message: {}", err);
                        }
                    }
                }
            }
            Some(Err(err)) => error!("Kafka error: {}", err),
            None => {}
        }
    }
}
