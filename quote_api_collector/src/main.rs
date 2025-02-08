use chrono::Local;
use fast_log::Config as FastLocaConfig;
use log::{error, info};
use rand::Rng;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};

const CURR_URL: &str = "http://flakysaas.com/currencies";
const CURR_USD: &str = "usd";
const QUOTE_URL: &str = "http://flakysaas.com/quote";

mod config;
use config::CONFIG;

#[derive(Serialize)]
struct QuoteRequest {
    quote: String,
    base: String,
}

#[derive(Deserialize, Debug)]
struct QuoteResponse {
    date: String,
    rate: f64,
    quote: String,
    base: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuoteEnvelope {
    date: String,
    rate: f64,
    quote: String,
    base: String,
    timestamp: u64,
}

impl From<QuoteResponse> for QuoteEnvelope {
    fn from(response: QuoteResponse) -> Self {
        Self {
            date: response.date,
            rate: response.rate,
            quote: response.quote,
            base: response.base,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub struct FileNameIdentifiers {
    pub time_stamp: String,
    pub random_nbr: u32,
}

fn setup_logger(fni: &FileNameIdentifiers) -> Result<(), Box<dyn std::error::Error>> {
    let file_path: String = format!(
        "logs/{}@{}@quote-api-collector-output.log",
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

fn create_kafka_producer() -> FutureProducer {
    match ClientConfig::new()
        .set("bootstrap.servers", &CONFIG.kafka.bootstrap_servers)
        .create::<FutureProducer>()
    {
        Ok(producer) => {
            info!("Kafka producer successfully created!");
            producer
        }
        Err(err) => {
            error!("Failed to create Kafka producer: {}", err);
            panic!("Kafka producer creation failed");
        }
    }
}

async fn fetch_data(url: &str) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await?
        .json::<Value>() // Parse as raw JSON
        .await?;

    Ok(response)
}

async fn fetch_quote_data(quote: &str, base: &str) -> Result<QuoteResponse, reqwest::Error> {
    let client = Client::new();

    let payload = QuoteRequest {
        quote: quote.to_string(),
        base: base.to_string(),
    };

    let response = client
        .post(QUOTE_URL)
        .json(&payload)
        .send()
        .await?
        .json::<QuoteResponse>()
        .await?;

    Ok(response)
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

    info!("Starting background process...");
    let producer: Rc<FutureProducer> = Rc::new(create_kafka_producer());

    let currencies = match fetch_data(CURR_URL).await {
        Ok(data) => {
            //println!("{:?}", data);
            if let Some(arr) = data.as_array() {
                let currencies: Vec<String> = arr
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                info!("Fetched currencies: {:?}", currencies);
                currencies
            } else {
                error!("Warning empty currency list");
                vec![]
            }
        }
        Err(err) => {
            error!("Error fetching data: {}", err);
            panic!("Closing system");
        }
    };

    loop {
        for curr in &currencies {
            match fetch_quote_data("usd", &curr).await {
                Ok(response) => {
                    info!(
                        "Date: {}, Exchange Rate from {} to {}: {}",
                        response.date, response.base, response.quote, response.rate
                    );

                    let envelope: QuoteEnvelope = response.into();
                    let json_payload = serde_json::to_string(&envelope).unwrap();
                    match producer
                        .send(
                            FutureRecord::to(&CONFIG.kafka.topic)
                                .key("") // Use key or empty string
                                .payload(&json_payload),
                            0, // Optional timeout
                        )
                        .await
                    {
                        Ok(_) => {
                            info!(
                                "Message |{}| sent successfully to topic '{}'",
                                &json_payload, &CONFIG.kafka.topic
                            );
                        }
                        Err(e) => {
                            error!("Failed to send message: {} ", json_payload);
                        }
                    }
                }
                Err(err) => {
                    error!("Error fetching quote for {}: {}", curr, err);
                }
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}
