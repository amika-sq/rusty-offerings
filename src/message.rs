pub mod close;
pub mod order;
pub mod order_status;
pub mod quote;
pub mod rfq;

use crate::message::close::CloseData;
use crate::message::order::OrderData;
use crate::message::order_status::OrderStatusData;
use crate::message::quote::QuoteData;
use crate::message::rfq::RfqData;

use crate::crypto::Crypto;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi_jwk::JWK;
use type_safe_id::{DynamicType, TypeSafeId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetadata {
    /// The message's ID
    pub id: String,
    /// This defines the data property's type (e.g. rfq, quote etc.)
    pub kind: String,
    /// ID for a "exchange" of messages between Alice <-> PFI.
    /// Set by the first message in an exchange.
    pub exchange_id: String,
    /// The sender's DID
    pub from: String,
    /// The recipient's DID
    pub to: String,
    /// ISO 8601
    pub created_at: String, // TODO: Change to chrono DateTime?
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageData {
    Close(CloseData),
    Order(OrderData),
    OrderStatus(OrderStatusData),
    Quote(QuoteData),
    Rfq(RfqData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedMessage {
    #[serde(flatten)]
    pub message: Message, // TODO:  Don't encapsulate, just re-define?
    /// Signature that verifies the authenticity and integrity of the message
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// An object containing fields about the message
    pub metadata: MessageMetadata,
    /// The actual message content
    pub data: MessageData,
}

impl Message {
    // Custom Constructor
    pub fn new(from: &String, to: &String, data: MessageData) -> Self {
        // Determine `kind` of message
        let kind = match &data {
            MessageData::Close(_) => "close".to_string(),
            MessageData::Order(_) => "order".to_string(),
            MessageData::OrderStatus(_) => "orderstatus".to_string(),
            MessageData::Quote(_) => "quote".to_string(),
            MessageData::Rfq(_) => "rfq".to_string(),
        };

        // Generate a TypeID, using `kind` as the prefix
        let dynamic_type = DynamicType::new(&kind).unwrap();
        let id = TypeSafeId::new_with_type(dynamic_type).to_string();

        let metadata = MessageMetadata {
            id: id.clone(),
            kind,
            exchange_id: id.clone(), // TODO : This is NOT the id in the case of non-rfq messages
            from: from.clone(),
            to: to.clone(),
            created_at: Utc::now().to_rfc3339(),
        };

        Message { metadata, data }
    }

    pub fn sign(self: Message, jwk: JWK, kid: String) -> SignedMessage {
        let payload = json!({
            "metadata": self.metadata,
            "data": self.data,
        });
        println!("payload: {}", payload);
        let payload_digest = Crypto::digest(payload).expect("Error computing digest.");

        let signature =
            Crypto::sign(&payload_digest, jwk, kid, true).expect("Error creating signature.");

        SignedMessage {
            message: self,
            signature,
        }
    }
}
