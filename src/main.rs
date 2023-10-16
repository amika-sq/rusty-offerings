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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Offerings Start ---

    // let url = "http://localhost:9000/offerings";
    //
    // let response = reqwest::get(url).await?.text().await?;
    //
    // let offerings: OfferingsResponse = serde_json::from_str(&response)?;
    //
    // println!(
    //     "{}",
    //     to_string_pretty(&offerings).expect("Failed to serialize to string")
    // );

    // --- Offerings End ---

    // --- RFQ Start ---

    // let rfq = devtools::create_rfq(offerings.data.first().unwrap()).await;
    // println!("{:?}", rfq);

    // --- RFQ End ---

    // --- Crypto hash start ---

    // let result = crypto::Crypto::hash("This is a whole different string, hoping the outputs still match!");
    // match result {
    //     Ok(hash) => println!("Result: {}", hash),
    //     Err(e) => println!("An error occurred: {}", e),
    // }

    // --- Crypto hash end ---

    // --- Sign Start ---

    let json_str = r#"
    {
        "d":"OhTqTBE4HvA2WzKe3o2izWtKr-wJcH7otL2SKivgOEE",
        "alg":"EdDSA",
        "crv":"Ed25519",
        "kty":"OKP",
        "ext":"true",
        "key_ops":["sign"],
        "x":"cAS02BpZ-Nfk8kL8GhlmYFD2V6U8pUZE1Ffwf5-hZJk"
    }
    "#;
    let jwk: JWK = serde_json::from_str(json_str)?;

    let kid = "did:key:z6MkmzXsQyZXrvYYoYh3K2Zc6p7JpT55EsjeTcVG6Gag7xf2#z6MkmzXsQyZXrvYYoYh3K2Zc6p7JpT55EsjeTcVG6Gag7xf2".to_string();
    let token = Crypto::sign("foobar", kid, jwk).expect("No token, sads");
    println!("Token: {:?}", token);

    /**
    kid: did:key:z6MkmzXsQyZXrvYYoYh3K2Zc6p7JpT55EsjeTcVG6Gag7xf2#z6MkmzXsQyZXrvYYoYh3K2Zc6p7JpT55EsjeTcVG6Gag7xf2
    privateKeyJwk: {"d":"OhTqTBE4HvA2WzKe3o2izWtKr-wJcH7otL2SKivgOEE","alg":"EdDSA","crv":"Ed25519","kty":"OKP","ext":"true","key_ops":["sign"],"x":"cAS02BpZ-Nfk8kL8GhlmYFD2V6U8pUZE1Ffwf5-hZJk"}
    Token: eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa216WHNReVpYcnZZWW9ZaDNLMlpjNnA3SnBUNTVFc2plVGNWRzZHYWc3eGYyI3o2TWttelhzUXlaWHJ2WVlvWWgzSzJaYzZwN0pwVDU1RXNqZVRjVkc2R2FnN3hmMiJ9.ImZvb2JhciI.aZzEOlW2WdHtF5-EI0zkFADTQYqi3JCN3cDpvc8jdvXEUlh0m09KsUYQf5snFcrN-XdmYYOp7Riw44W_U5aFDg
    */
    // --- Sign End ---
    Ok(())
}
