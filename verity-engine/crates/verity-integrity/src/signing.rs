use ed25519_dalek::{Signer, Verifier, SigningKey, VerifyingKey, Signature};
use serde::Serialize;
use verity_kernel::{VerityError, canonical_serialize};

/// Sign a VerityReceipt with the engine's Ed25519 key.
/// Invariant 2: Objects are signed before any effect.
pub struct VeritySigner {
    signing_key: SigningKey,
}

impl VeritySigner {
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    pub fn generate() -> Self {
        let secret: [u8; 32] = rand::random();
        let signing_key = SigningKey::from_bytes(&secret);
        Self { signing_key }
    }

    /// Sign content and return base64-encoded signature prefixed with "ed25519:"
    pub fn sign(&self, content: &impl Serialize) -> Result<String, VerityError> {
        let bytes = canonical_serialize(content)?;
        let signature = self.signing_key.sign(&bytes);
        let sig_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            signature.to_bytes(),
        );
        Ok(format!("ed25519:{}", sig_b64))
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}

/// Verify a signature against a public key.
pub fn verify_signature(
    public_key: &VerifyingKey,
    content: &impl Serialize,
    signature_str: &str,
) -> Result<bool, VerityError> {
    let sig_b64 = signature_str
        .strip_prefix("ed25519:")
        .ok_or_else(|| VerityError::SignatureError("missing ed25519: prefix".to_string()))?;

    let sig_bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        sig_b64,
    )
    .map_err(|e| VerityError::SignatureError(format!("base64 decode: {}", e)))?;

    let signature = Signature::from_bytes(
        sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| VerityError::SignatureError("invalid signature length".to_string()))?,
    );

    let bytes = canonical_serialize(content)?;
    Ok(public_key.verify(&bytes, &signature).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sign_and_verify_roundtrip() {
        let signer = VeritySigner::generate();
        let content = json!({"decision": "release_funds", "amount": 500});

        let signature = signer.sign(&content).unwrap();
        assert!(signature.starts_with("ed25519:"));

        let verified = verify_signature(&signer.public_key(), &content, &signature).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_wrong_key_fails() {
        let signer1 = VeritySigner::generate();
        let signer2 = VeritySigner::generate();
        let content = json!({"decision": "release_funds"});

        let signature = signer1.sign(&content).unwrap();
        let verified = verify_signature(&signer2.public_key(), &content, &signature).unwrap();
        assert!(!verified);
    }

    #[test]
    fn test_wrong_content_fails() {
        let signer = VeritySigner::generate();
        let content1 = json!({"decision": "release_funds"});
        let content2 = json!({"decision": "refund"});

        let signature = signer.sign(&content1).unwrap();
        let verified = verify_signature(&signer.public_key(), &content2, &signature).unwrap();
        assert!(!verified);
    }

    #[test]
    fn test_invalid_prefix() {
        let signer = VeritySigner::generate();
        let content = json!({"test": true});

        assert!(verify_signature(&signer.public_key(), &content, "bad:signature").is_err());
    }
}
