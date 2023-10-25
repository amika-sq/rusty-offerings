mod crypto;
mod devtools;
mod message;
mod offering;

use offering::OfferingsResponse;
use reqwest;
use std::error::Error;

use crate::crypto::Crypto;
use serde_json::to_string_pretty;
use ssi_jwk::{Algorithm, Base64urlUInt, ECParams, OctetParams, JWK};

use crate::devtools::create_rfq;
use crate::message::rfq::{PaymentMethod, RfqData};
use crate::message::{Data, Message, SignedMessage};
use crate::offering::Offering;
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

async fn get_offerings() -> Result<Vec<Offering>, Box<dyn Error>> {
    let url = "http://localhost:9000/offerings";
    let response = reqwest::get(url).await?.text().await?;
    let offerings: OfferingsResponse = serde_json::from_str(&response)?;

    Ok(offerings.data)
}
