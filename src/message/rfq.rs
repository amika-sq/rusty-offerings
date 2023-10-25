use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rfq {
    /// Offering which Alice would like to get a quote for
    pub offering_id: String,
    /// Specify which payment method to send payin currency
    pub payin_method: PaymentMethod,
    /// Specify which payment method to receive payout currency
    pub payout_method: PaymentMethod,
    /// Amount of payin currency you want in exchange for payout currency
    pub payin_subunits: String,
    /// An array of claims that fulfill the requirements declared in an Offering
    pub claims: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    /// Type of payment method (i.e. DEBIT_CARD, BITCOIN_ADDRESS, SQUARE_PAY)
    pub kind: String,
    /// An object containing the properties defined in an Offering's `requiredPaymentDetails` json schema
    pub payment_details: HashMap<String, String>,
}
