use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingsResponse {
    data: Vec<Offering>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Offering {
    metadata: Metadata,
    data: OfferingData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    from: String,
    kind: String,
    id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingData {
    description: String,
    #[serde(rename = "payinCurrency")]
    payin_currency: Currency,
    #[serde(rename = "payoutCurrency")]
    payout_currency: PayoutCurrency,
    #[serde(rename = "payoutUnitsPerPayinUnit")]
    payout_units_per_payin_unit: String,
    #[serde(rename = "payinMethods")]
    payin_methods: Vec<PaymentMethod>,
    #[serde(rename = "payoutMethods")]
    payout_methods: Vec<PaymentMethod>,
    #[serde(rename = "requiredClaims")]
    required_claims: RequiredClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayoutCurrency {
    #[serde(rename = "currencyCode")]
    currency_code: String,
    #[serde(rename = "maxSubunits")]
    max_subunits: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    kind: String,
    #[serde(rename = "requiredPaymentDetails")]
    required_payment_details: PaymentDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetails {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "type")]
    type_field: String,
    properties: std::collections::HashMap<String, PropertyDetails>,
    required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    additional_properties: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDetails {
    #[serde(rename = "type")]
    type_field: String,
    description: String,
    #[serde(rename = "minLength")]
    min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    max_length: Option<u32>,
    pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequiredClaims {
    id: String,
    #[serde(rename = "input_descriptors")]
    input_descriptors: Vec<InputDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputDescriptor {
    id: String,
    constraints: Constraint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraint {
    fields: Vec<FieldConstraint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldConstraint {
    path: Vec<String>,
    filter: Filter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(rename = "const")]
    const_field: String,
}
