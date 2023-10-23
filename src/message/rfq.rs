use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RfqData {
    pub offering_id: String,
    pub payin_method: PaymentMethod,
    pub payout_method: PaymentMethod,
    pub payin_subunits: String,
    pub claims: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub kind: String,
    pub payment_details: HashMap<String, String>,
}
