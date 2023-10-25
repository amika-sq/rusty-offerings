pub mod offering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// The resource's ID
    pub id: String,
    /// The author's DID
    pub from: String,
    /// The `data` property's type. (e.g. `offering`)
    pub kind: String,
    /// ISO 8601 timestamp
    pub created_at: String, // TODO: Change to chrono Datetime?
    /// ISO 8601 timestamp
    pub updated_at: Option<String>,
}

trait Data<'de>: Serialize + Deserialize<'de> {
    fn kind() -> String;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resource<T: Data<'static>> {
    /// An object containing fields _about_ the resource
    pub metadata: Metadata,
    /// The actual resource content (e.g. an offering)
    pub data: T,
    /// Signature that verifies the authenticity and integrity of the message
    pub signature: String,
}
