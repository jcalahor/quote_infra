use chrono::Local;
use fast_log::Config as FastLocaConfig;
use log::{error, info};
use rand::Rng;

use rdkafka::{
    consumer::{BaseConsumer, Consumer},
    ClientConfig, Message as RdMesssage,
};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
mod config;
use config::CONFIG;

#[derive(Serialize, Deserialize, Debug)]
struct QuoteEnvelope {
    date: String,
    rate: f64,
    quote: String,
    base: String,
    timestamp: u64,
}

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

pub struct FileNameIdentifiers {
    pub time_stamp: String,
    pub random_nbr: u32,
}

fn setup_logger(fni: &FileNameIdentifiers) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = format!(
        "logs/{}@{}@quote-sinker-output.log",
        fni.time_stamp.clone(),
        fni.random_nbr
    );
    fast_log::init(
        FastLocaConfig::new()
            .file(file_path.as_str())
            .chan_len(Some(10000))
            .level(log::LevelFilter::Info),
    )
    .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=1000);
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    let fni = FileNameIdentifiers {
        time_stamp: timestamp,
        random_nbr: random_number,
    };
    setup_logger(&fni).expect("Failed to set up logger");

    let consumer = create_kafka_consumer();
    consumer
        .subscribe(&[&CONFIG.kafka.topic])
        .expect("topic subscribe failed");

    info!("Starting background process...");
    loop {
        match consumer.poll(Duration::from_secs(1)) {
            Some(Ok(msg)) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    match payload {
                        Ok(json) => match serde_json::from_str::<QuoteEnvelope>(json) {
                            Ok(quote) => {
                                info!("Received Quote: {:?}", quote);
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
