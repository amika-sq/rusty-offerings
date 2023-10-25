use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteData {
    pub expires_at: String, // TODO: Change to chrono datetime?
    pub payin: QuoteDetails,
    pub payout: QuoteDetails,
    pub payment_instructions: PaymentInstructions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteDetails {
    pub currency_code: String,
    pub amount_subunits: String, // TODO: Look at JS/KT to see what type this actually is
    pub fee_subunits: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstructions {
    payin: PaymentInstruction,
    payout: PaymentInstruction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstruction {
    pub link: String,
    pub instruction: String,
}
