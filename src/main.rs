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

use crate::message::rfq::{PaymentMethod, RfqData};
use crate::message::{Data, Message};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let rfq_data = RfqData {
    //     offering_id: "1".to_string(),
    //     payin_method: PaymentMethod {
    //         kind: "DEBIT".to_string(),
    //         payment_details: HashMap::from([(
    //             "btcAddress".to_string(),
    //             "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    //         )]),
    //     },
    //     payout_method: PaymentMethod {
    //         kind: "DEBIT".to_string(),
    //         payment_details: HashMap::from([(
    //             "btcAddress".to_string(),
    //             "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    //         )]),
    //     },
    //     payin_subunits: "USD".to_string(),
    //     claims: vec!["claim1".to_string()],
    // };
    //
    let json_str = r#"
        {"d":"NdVaemsIWLXDYf3jfIqo_2IlI2j68btbavin_I9CbrM","alg":"EdDSA","crv":"Ed25519","kty":"OKP","ext":"true","key_ops":["sign"],"x":"yQ-KjEGgaIgY0hcDJ79zBtje1fn4T3VnTjUV14ki18w"}    
    "#;
    let jwk: JWK = serde_json::from_str(json_str)?;
    let kid = "did:key:z6Mksz7jj2bChDT8fg6VzYURcSQyb94BgViw5E51RdGSQYDq#z6Mksz7jj2bChDT8fg6VzYURcSQyb94BgViw5E51RdGSQYDq".to_string();

    let signature = Crypto::sign(&"foobar".to_string().into_bytes(), jwk, kid, true)
        .expect("Unable to gen signature");
    println!("Signature: {}", signature);

    Ok(())
}
