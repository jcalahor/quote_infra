use reqwest::Client;
use tokio::time::{sleep, Duration};
use serde_json::Value;
use serde::{Deserialize, Serialize};

const CURR_URL:&str = "http://flakysaas.com/currencies";
const CURR_USD:&str = "usd";
const QUOTE_URL: &str = "http://flakysaas.com/quote";



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

async fn fetch_data(url: &str) -> Result<Value, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url)
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

    let response = client.post(QUOTE_URL)
        .json(&payload)
        .send()
        .await?
        .json::<QuoteResponse>() 
        .await?;

    Ok(response)
}


#[tokio::main]
async fn main() {
    println!("Starting background process...");

    let currencies = match fetch_data(CURR_URL).await {
        Ok(data) => {
            //println!("{:?}", data);
            if let Some(arr) = data.as_array() {
                let currencies: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                println!("Fetched currencies: {:?}", currencies);
                currencies
            } else {
                println!("Warning empty currency list");
                vec![]
            }
        }
        Err(err) => {
            eprintln!("Error fetching data: {}", err);
            panic!("Closing system");
        }
    };

    loop {
        for curr in &currencies {
            match fetch_quote_data( "usd", &curr).await {
                Ok(response) => {
                    println!(
                        "Date: {}, Exchange Rate from {} to {}: {}",
                        response.date, response.base, response.quote, response.rate
                    );
                }
                Err(err) => {
                    eprintln!("Error fetching quote for {}: {}", curr, err);
                }
            }
        }

        sleep(Duration::from_secs(10)).await; 
    }
}
