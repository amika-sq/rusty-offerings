mod devtools;
mod offering;

use offering::OfferingsResponse;
use reqwest;
use std::error::Error;

use serde_json::to_string_pretty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9000/offerings";

    let response = reqwest::get(url).await?.text().await?;

    let offerings: OfferingsResponse = serde_json::from_str(&response)?;

    println!(
        "{}",
        to_string_pretty(&offerings).expect("Failed to serialize to string")
    );

    // devtools::create_rfq(offerings.data.first().unwrap()).await;

    Ok(())
}
