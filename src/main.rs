mod offering;

use reqwest;
use std::error::Error;
use offering::OfferingsResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9000/offerings";

    let response = reqwest::get(url).await?.text().await?;

    let offerings: OfferingsResponse = serde_json::from_str(&response)?;

    println!("{:?}", offerings); // Use Debug print for the deserialized response

    Ok(())
}