use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::http::header::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use tari_utilities::hex;

use crate::error::ServerError;

pub fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or_else(|_| "0000000".to_string(), |d| d.as_secs().to_string())
}

pub fn check_valid_signature(secret: &str, signature: &str, payload: &str) -> Result<(), ServerError> {
    if !signature.starts_with("sha256=") {
        return Err(ServerError::InvalidSignatureHeader(
            "Signature does not start with \"sha256=\"".into(),
        ));
    }
    let hash = generate_hash(secret, payload)?;
    let sig = hex::from_hex(&signature[7..]).map_err(|e| ServerError::InvalidSignatureHeader(e.to_string()))?;
    match hash.ct_eq(&sig).unwrap_u8() {
        1 => Ok(()),
        0 => Err(ServerError::InvalidSignature),
        _ => Err(ServerError::Unspecified(
            "Signature comparison returned unexpected result".into(),
        )),
    }
}

pub fn get_secret() -> Result<String, ServerError> {
    std::env::var("GH_PILOT_WEBHOOK_SECRET")
        .map_err(|_| ServerError::InvalidSecret("GH_PILOT_WEBHOOK_SECRET is not set".into()))
}

pub fn extract_signature(headers: &HeaderMap) -> Result<&str, ServerError> {
    headers
        .get("x-hub-signature-256")
        .ok_or_else(|| ServerError::InvalidSignatureHeader("x-hub-signature-256 is missing".into()))?
        .to_str()
        .map_err(|_| ServerError::InvalidSignatureHeader("x-hub-signature-256 is not a valid string".into()))
}

pub fn generate_hash(secret: &str, payload: &str) -> Result<Vec<u8>, ServerError> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| {
        ServerError::InvalidSecret("Please reconfigure webhook delivery with a new secret string.".into())
    })?;
    mac.update(payload.as_bytes());
    let hash = mac.finalize().into_bytes().to_vec();
    Ok(hash)
}

#[cfg(test)]
mod test {
    use crate::{error::ServerError, utilities::check_valid_signature};

    #[test]
    fn valid_signature() {
        // Event 83308ad0-34cd-11ed-87f6-b93609568f2c
        let sig = "sha256=4a46a5402575f847b95322c07386afc14f78c34ebd41873e9e35e142f8039e3d";

        let payload = include_str!("../test-data/signature_payload.json");
        let secret = "this_is_my_very_safe_secret";
        match check_valid_signature(secret, sig, payload) {
            Ok(()) => (),
            Err(e) => panic!("Signature validation failed: {}", e),
        }
    }

    #[test]
    fn invalid_signature() {
        let sig = "sha256=123456402575f847b95322c07386afc14f78c34ebd41873e9e35e142f8039e3d";

        let payload = include_str!("../test-data/signature_payload.json");
        let secret = "this_is_my_very_safe_secret";
        match check_valid_signature(secret, sig, payload) {
            Err(ServerError::InvalidSignature) => {},
            Err(e) => panic!("Signature validation should fail with invalid signature, but got {}", e),
            Ok(()) => panic!("Signature validation should fail"),
        }
    }

    #[test]
    fn invalid_signature_headers() {
        let payload = "foo";
        let secret = "secret";

        match check_valid_signature(secret, "sha1=123456", payload) {
            Err(ServerError::InvalidSignatureHeader(s)) => assert_eq!(s, "Signature does not start with \"sha256=\""),
            Err(e) => panic!("Signature header should be invalid, but got '{}'", e),
            Ok(()) => panic!("Signature validation should fail"),
        }

        match check_valid_signature(secret, "sha256=12345", payload) {
            Err(ServerError::InvalidSignatureHeader(s)) => assert_eq!(s, "Hex string lengths must be a multiple of 2"),
            Err(e) => panic!("Signature header should be invalid, but got '{}'", e),
            Ok(()) => panic!("Signature validation should fail"),
        }
    }
}
