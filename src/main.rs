mod offering;

use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9000/offerings";

    let response = reqwest::get(url).await?.text().await?;

    println!("Received response:\n{}", response);

    Ok(())
}