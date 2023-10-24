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
use crate::message::{Data, Message, SignedMessage};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rfq_message = create_hardcoded_rfq_message()?;
    Ok(())
}

fn create_hardcoded_rfq_message() -> Result<SignedMessage, Box<dyn Error>> {
    let jwk_json_str = r#"
        {"d":"kPKqPIB5Nkv-gEpUr-9Ayqm5DFgGmX02WOdpleBFTME","alg":"EdDSA","crv":"Ed25519","kty":"OKP","ext":"true","key_ops":["sign"],"x":"lWEi7j72-LM89wIcNrnLhlwHl_a69okubkhjEEVdRlw"}    
    "#;
    let jwk: JWK = serde_json::from_str(jwk_json_str)?;
    let kid = "did:key:z6MkpWNhaSbkUS1UJ9DuuRPzPzMHAE69Shm5YzAdb9kcYKDu#z6MkpWNhaSbkUS1UJ9DuuRPzPzMHAE69Shm5YzAdb9kcYKDu".to_string();

    let rfq_data = RfqData {
        offering_id: "1".to_string(),
        payin_method: PaymentMethod {
            kind: "DEBIT".to_string(),
            payment_details: HashMap::from([
                ("cardNumber".to_string(), "1234567890123456".to_string()),
                ("expiryDate".to_string(), "12/22".to_string()),
                (
                    "cardHolderName".to_string(),
                    "Ephraim Bartholomew Winthrop".to_string(),
                ),
                ("cvv".to_string(), "123".to_string()),
            ]),
        },
        payout_method: PaymentMethod {
            kind: "BTC_ADDRESS".to_string(),
            payment_details: HashMap::from([(
                "btcAddress".to_string(),
                "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            )]),
        },
        payin_subunits: "20000".to_string(),
        claims: vec!["claim1".to_string()],
    };

    let rfq_message = Message::new(Data::Rfq(rfq_data));
    let signed_rfq_message = rfq_message.sign(jwk, kid);

    println!("RFQ Message Signature: {}", signed_rfq_message.signature);

    Ok(signed_rfq_message)
}
