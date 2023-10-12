use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingsResponse {
    pub data: Vec<Offering>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offering {
    pub metadata: Metadata,
    pub data: OfferingData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub from: String,
    pub kind: String,
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingData {
    pub description: String,
    #[serde(rename = "payinCurrency")]
    pub payin_currency: Currency,
    #[serde(rename = "payoutCurrency")]
    pub payout_currency: PayoutCurrency,
    #[serde(rename = "payoutUnitsPerPayinUnit")]
    pub payout_units_per_payin_unit: String,
    #[serde(rename = "payinMethods")]
    pub payin_methods: Vec<PaymentMethod>,
    #[serde(rename = "payoutMethods")]
    pub payout_methods: Vec<PaymentMethod>,
    #[serde(rename = "requiredClaims")]
    pub required_claims: RequiredClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayoutCurrency {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "maxSubunits")]
    pub max_subunits: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub kind: String,
    #[serde(rename = "requiredPaymentDetails")]
    pub required_payment_details: PaymentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetails {
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: std::collections::HashMap<String, PropertyDetails>,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDetails {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    #[serde(rename = "minLength")]
    pub min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<u32>,
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequiredClaims {
    pub id: String,
    #[serde(rename = "input_descriptors")]
    pub input_descriptors: Vec<InputDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptor {
    pub id: String,
    pub constraints: Constraint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraint {
    pub fields: Vec<FieldConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldConstraint {
    pub path: Vec<String>,
    pub filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "const")]
    pub const_field: String,
}
