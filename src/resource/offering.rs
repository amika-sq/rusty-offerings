use crate::resource::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offering {
    pub description: String,
    pub payin_currency: Currency,
    pub payout_currency: PayoutCurrency,
    pub payout_units_per_payin_unit: String,
    pub payin_methods: Vec<PaymentMethod>,
    pub payout_methods: Vec<PaymentMethod>,
    pub required_claims: RequiredClaims,
}

impl Data<'_> for Offering {
    fn kind() -> String {
        "offering".to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutCurrency {
    pub currency_code: String,
    pub max_subunits: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub kind: String,
    pub required_payment_details: PaymentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDetails {
    pub schema: String,
    pub type_field: String,
    pub properties: std::collections::HashMap<String, PropertyDetails>,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDetails {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
    pub min_length: Option<u32>,
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
    pub const_field: String,
}
