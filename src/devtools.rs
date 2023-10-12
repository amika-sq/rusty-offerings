use crate::offering::Offering;

use serde_derive::{Deserialize, Serialize};
use ssi_vc::Credential;
use std::collections::HashMap;

use didkit::{DIDCreate, Error as DidKitError, ResolutionInputMetadata, Source, DID_METHODS, JWK};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rfq {
    pub metadata: Metadata,
    pub data: RfqData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RfqData {
    pub offering_id: String,
    pub payin_method: PaymentMethod,
    pub payout_method: PaymentMethod,
    pub payin_subunits: String,
    pub claims: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub kind: String,
    pub payment_details: HashMap<String, String>,
}

#[allow(unused)]
pub async fn create_rfq(offering: &Offering) {
    // TODO: Create DIDs for sender of RFQ and issuer of Credential

    let did_key_method = DID_METHODS
        .get("key") // TODO: This is gross, can I use the constant somehow?
        .ok_or(DidKitError::UnknownDIDMethod)
        .unwrap();
    let did_key_resolver = did_key_method.to_resolver();

    let issuer_key = JWK::generate_secp256k1().unwrap();
    let issuer_did = did_key_method.generate(&Source::Key(&issuer_key)).unwrap();

    let subject_key = JWK::generate_secp256k1().unwrap();
    let subject_did = did_key_method.generate(&Source::Key(&issuer_key)).unwrap();

    let vc_str = format!(
        r###"{{
            "@context": "https://www.w3.org/2018/credentials/v1",
            "id": "http://example.org/credentials/3731",
            "type": ["VerifiableCredential", "YoloCredential"],
            "issuer": "{}",
            "issuanceDate": "2020-08-19T21:41:50Z",
            "credentialSubject": {{
                "id": "{}"
            }}
        }}"###,
        issuer_did, subject_did
    );

    let vc = Credential::from_json_unsigned(&vc_str).unwrap();
    let mut proof_options = ssi_vc::LinkedDataProofOptions::default();
    proof_options.checks = None;
    proof_options.created = None;
    let jwt_string = vc
        .generate_jwt(Some(&issuer_key), &proof_options, did_key_resolver)
        .await
        .unwrap();
    println!("Generated JWT String: {:?}", jwt_string);

    // let sender = &opts.sender;
    //
    // let credential_result = Self::create_credential(CreateCredentialOptions {
    //     type_field: "YoloCredential".to_string(),
    //     issuer: sender.clone(),
    //     subject: sender.did.clone(),
    //     data: [("beep".to_string(), "boop".to_string())]
    //         .iter()
    //         .cloned()
    //         .collect(),
    // })
    // .await?;
    //
    //
    // let rfq_data = RfqData {
    //     offering_id: offering.metadata.id,
    //     payin_method: PaymentMethod {
    //         kind: "DEBIT_CARD".to_string(),
    //         payment_details: [
    //             ("cardNumber".to_string(), "1234567890123456".to_string()),
    //             ("expiryDate".to_string(), "12/22".to_string()),
    //             (
    //                 "cardHolderName".to_string(),
    //                 "Ephraim Bartholomew Winthrop".to_string(),
    //             ),
    //             ("cvv".to_string(), "123".to_string()),
    //         ]
    //         .iter()
    //         .cloned()
    //         .collect(),
    //     },
    //     payout_method: PaymentMethod {
    //         kind: "BTC_ADDRESS".to_string(),
    //         payment_details: [(
    //             "btcAddress".to_string(),
    //             "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    //         )]
    //         .iter()
    //         .cloned()
    //         .collect(),
    //     },
    //     payin_subunits: "20000".to_string(),
    //     // claims: vec![credential_result.signed_credential],
    //     claims: vec![],
    // };
    // //
    // // Ok(Rfq {
    // //     metadata: Metadata {
    // //         from: sender.did.clone(),
    // //         to: "did:ex:pfi".to_string(),
    // //     },
    // //     data: rfq_data,
    // // })
    // Ok(Rfq {
    //     metadata: Metadata {
    //         from: "farts".to_string(), // TODO: from sender's did
    //         to: "farts".to_string(),   // TODO: How do I get the PFI's did?
    //     },
    //     data: rfq_data,
    // })
}
