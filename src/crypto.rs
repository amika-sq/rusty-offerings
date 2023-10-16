use base64::engine::general_purpose::URL_SAFE_NO_PAD as base64;
use base64::Engine;
use ciborium;
use serde::Serialize;
use sha2::{Digest, Sha256};
use ssi_jwk::{Params, JWK};
use ssi_jws::{encode_sign_custom_header, Header};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CRV_TO_ALG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Ed25519", "EdDSA");
        map.insert("secp256k1", "ES256K");
        map
    };
}

pub struct Crypto;

impl Crypto {
    // /**
    // Hashes the payload provided in the following manner:
    // ```
    // base64url(
    //     sha256(
    //         cbor(payload)
    //     )
    // )
    // ```
    // */
    // pub fn hash<T: Serialize>(payload: T) -> Result<String, Box<dyn std::error::Error>> {
    //     // CBOR encode the payload
    //     let mut cbor_payload = Vec::new();
    //     ciborium::ser::into_writer(&payload, &mut cbor_payload).expect("Failed to CBOR encode");
    //
    //     // SHA-256 hash the CBOR encoded payload
    //     let mut hasher = Sha256::new();
    //     hasher.update(cbor_payload);
    //     let sha256_payload = hasher.finalize();
    //
    //     // Base64URL encode the Sha-256 hashed payload
    //     let base64_payload = base64.encode(sha256_payload);
    //     Ok(base64_payload)
    // }

    pub fn sign<T: Serialize>(
        payload: T,
        kid: String,
        jwk: JWK,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let payload_str = &serde_json::to_string(&payload)?;

        let header = Header {
            algorithm: jwk.algorithm.expect("Can't determine signing algorithm."),
            key_id: Some(kid),
            ..Default::default()
        };
        Ok(encode_sign_custom_header(payload_str, &jwk, &header)?)
    }
}
