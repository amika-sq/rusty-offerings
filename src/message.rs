pub mod rfq;

use crate::message::rfq::RfqData;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use type_safe_id::{DynamicType, TypeSafeId};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    id: String,
    kind: String,
    exchange_id: String,
    from: String,
    to: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Data {
    Rfq(RfqData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedMessage {
    #[serde(flatten)]
    message: Message,
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    metadata: Metadata,
    data: Data,
}

impl Message {
    // Custom Constructor
    pub fn new(data: Data) -> Self {
        // Determine `kind` of message
        let kind = match &data {
            Data::Rfq(_) => "rfq".to_string(),
        };

        // Generate a TypeID, using `kind` as the prefix
        let dynamic_type = DynamicType::new(&kind).unwrap();
        let id = TypeSafeId::new_with_type(dynamic_type).to_string();

        let metadata = Metadata {
            id: id.clone(),
            kind,
            exchange_id: id.clone(), // TODO : This is NOT the id in the case of non-rfq messages
            from: "alice".to_string(),
            to: "bob".to_string(),
            created_at: Utc::now().to_rfc3339(),
        };

        Message { metadata, data }
    }

    pub fn sign(self) -> SignedMessage {
        SignedMessage {
            message: self,
            signature: "123".to_string(),
        }
    }
}
