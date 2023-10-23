use base64::Engine;
use ciborium;
use serde::Serialize;
use sha2::{Digest, Sha256};
use ssi_jwk::{Params, JWK};
use ssi_jws::{encode_sign_custom_header, Header};
use ssi_vc::base64_encode_json;

pub struct Crypto;

impl Crypto {
    pub fn digest<T: Serialize>(payload: T) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let canonicalized = serde_jcs::to_string(&payload).unwrap();
        let digest = Sha256::new()
            .chain_update(canonicalized)
            .finalize()
            .to_vec();

        Ok(digest)
        // Ok("ok".to_string().into_bytes())
    }

    pub fn sign(
        payload: &[u8],
        jwk: JWK,
        kid: String,
        detached: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let algorithm = jwk.algorithm.expect("Unable to determine algorithm");
        let jws_header = Header {
            algorithm,
            key_id: Some(kid),
            ..Default::default()
        };

        let encoded_jws_header =
            base64_encode_json(&jws_header).expect("Unable to encode JWS Header");
        let encoded_jws_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload);

        let to_sign = format!("{}.{}", encoded_jws_header, encoded_jws_payload);
        let to_sign_bytes = to_sign.into_bytes();

        let encoded_signature =
            ssi_jws::sign_bytes_b64(algorithm, &to_sign_bytes, &jwk).expect("Failed to sign.");

        Ok(if detached {
            format!("{}..{}", encoded_jws_header, encoded_signature)
        } else {
            format!(
                "{}.{}.{}",
                encoded_jws_header, encoded_jws_payload, encoded_signature
            )
        })
    }
}
