use crate::offering::Offering;

use serde_derive::{Deserialize, Serialize};
use ssi_vc::Credential;
use std::collections::HashMap;

use crate::message::rfq::{PaymentMethod, RfqData};
use crate::message::{Data, Message, SignedMessage};
use didkit::{DIDCreate, Error as DidKitError, ResolutionInputMetadata, Source, DID_METHODS, JWK};

#[allow(unused)]
pub async fn create_rfq(offering: &Offering) -> SignedMessage {
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

    let rfq_data = RfqData {
        offering_id: offering.metadata.id.clone(),
        payin_method: PaymentMethod {
            kind: "DEBIT_CARD".to_string(),
            payment_details: [
                ("cardNumber".to_string(), "1234567890123456".to_string()),
                ("expiryDate".to_string(), "12/22".to_string()),
                (
                    "cardHolderName".to_string(),
                    "Ephraim Bartholomew Winthrop".to_string(),
                ),
                ("cvv".to_string(), "123".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        payout_method: PaymentMethod {
            kind: "BTC_ADDRESS".to_string(),
            payment_details: [(
                "btcAddress".to_string(),
                "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),
        },
        payin_subunits: "20000".to_string(),
        claims: vec![jwt_string],
    };

    let rfq_message = Message::new(&subject_did, &offering.metadata.from, Data::Rfq(rfq_data));
    let signed_rfq_message = rfq_message.sign(subject_key, "lkwjelkj".to_string());

    signed_rfq_message
}
