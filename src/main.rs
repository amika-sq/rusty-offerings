mod crypto;
mod devtools;
mod message;
mod resource;

use reqwest;
use std::error::Error;

use crate::crypto::Crypto;
use serde_json::to_string_pretty;
use ssi_jwk::{Algorithm, Base64urlUInt, ECParams, OctetParams, JWK};

use crate::devtools::create_rfq;
use crate::message::rfq::{PaymentMethod, Rfq};
use crate::message::{Message, MessageData, SignedMessage};
use crate::resource::offering::Offering;
use crate::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let offerings = get_offerings().await?;
    let first_offering = offerings.first().expect("Panic, no offerings");
    let rfq_message = create_rfq(first_offering).await;

    println!(
        "RFQ Message: {}",
        serde_json::to_string_pretty(&rfq_message)?
    );

    let url = format!(
        "http://localhost:9000/exchanges/{}/rfq",
        rfq_message.message.metadata.id
    );
    println!("Sending RFQ to: {}", &url);

    let body = serde_json::to_string(&rfq_message)?;
    println!("Request body: {}", &body);

    let client = reqwest::Client::new();
    let res = client.post(&url).json(&rfq_message).send().await?;
    println!("Send RFQ response: {:?}", res);

    Ok(())
}

async fn get_offerings() -> Result<Vec<Resource<Offering>>, Box<dyn Error>> {
    let url = "http://localhost:9000/offerings";
    let response = reqwest::get(url).await?.text().await?;
    let offerings: OfferingsResponse = serde_json::from_str(&response)?;

    Ok(offerings.data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingsResponse {
    pub data: Vec<Resource<Offering>>,
}
