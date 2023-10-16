use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// RFQ SPECIFIC STUFF

#[derive(Debug, Serialize, Deserialize)]
pub struct RfqData {
    pub offering_id: String,
    pub payin_method: PaymentMethod,
    pub payout_method: PaymentMethod,
    pub payin_subunits: String,
    pub claims: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub kind: String,
    pub payment_details: HashMap<String, String>,
}

// THIS IS ACTUALLY MESSAGE

pub trait ResourceDataType {
    fn kind() -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The message ID
    id: String,
    /// The message kind
    kind: String,
    /// The exchange ID
    exchange_id: String,
    /// The sender's DID
    from: String,
    /// The recipient's DID
    to: String,
    /// Message creation time, expressed as ISO8601
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Rfq(RfqData),
}

pub struct Message {
    pub metadata: Metadata,
    pub data: Data,
    pub signature: String,
}

impl Message {
    pub fn new(data: Data) -> Self {
        let kind = match &data {
            Data::Rfq(_) => "rfq".to_string(),
        };

        let metadata = Metadata {
            id: "farts".to_string(),
            kind,
            exchange_id: "farts".to_string(),
            from: "farts".to_string(),
            to: "farts".to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        Message {
            metadata,
            data,
            signature: "farts".to_string(),
        }
    }
}
