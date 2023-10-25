use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    /// When this quote expires. Expressed as ISO8601
    pub expires_at: String, // TODO: Change to chrono datetime?
    /// The amount of payin currency that the PFI will receive
    pub payin: QuoteDetails,
    /// The amount of payout currency that Alice will receive
    pub payout: QuoteDetails,
    /// Object that describes how to pay the PFI, and how to get paid by the PFI
    /// (e.g. BTC address, payment link)
    pub payment_instructions: PaymentInstructions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteDetails {
    /// ISO 3166 currency code string
    pub currency_code: String,
    /// The amount of currency expressed in the smallest respective unit
    pub amount_subunits: String, // TODO: Look at JS/KT to see what type this actually is
    /// The amount paid in fees
    pub fee_subunits: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstructions {
    /// Link or Instruction describing how to pay the PFI
    payin: PaymentInstruction,
    /// Link or Instruction describing how to get paid by the PFI
    payout: PaymentInstruction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstruction {
    /// Link to allow Alice to pay PFI, or be paid by the PFI
    pub link: String,
    /// Instruction on how Alice can pay PFI, or how Alice can be paid by the PFI
    pub instruction: String,
}
