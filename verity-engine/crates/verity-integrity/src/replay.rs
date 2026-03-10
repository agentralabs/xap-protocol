use serde::Serialize;
use sha2::{Sha256, Digest};
use verity_kernel::{VerityError, canonical_serialize};

/// Compute the replay_hash for a VerityReceipt.
/// replay_hash = SHA-256(canonical_serialize(input_state) + canonical_serialize(rules_applied) + canonical_serialize(computation))
pub fn compute_replay_hash(
    input_state: &impl Serialize,
    rules_applied: &impl Serialize,
    computation: &impl Serialize,
) -> Result<String, VerityError> {
    let input_bytes = canonical_serialize(input_state)?;
    let rules_bytes = canonical_serialize(rules_applied)?;
    let comp_bytes = canonical_serialize(computation)?;

    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    hasher.update(&rules_bytes);
    hasher.update(&comp_bytes);
    let hash = hasher.finalize();

    Ok(format!("sha256:{}", hex::encode(hash)))
}

/// Verify a replay_hash matches the given components.
pub fn verify_replay_hash(
    claimed_hash: &str,
    input_state: &impl Serialize,
    rules_applied: &impl Serialize,
    computation: &impl Serialize,
) -> Result<bool, VerityError> {
    let computed = compute_replay_hash(input_state, rules_applied, computation)?;
    Ok(computed == claimed_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compute_and_verify_roundtrip() {
        let input = json!({"state": "PENDING_VERIFICATION"});
        let rules = json!({"version": "0.2.0"});
        let comp = json!({"steps": [{"op": "check"}]});

        let hash = compute_replay_hash(&input, &rules, &comp).unwrap();
        assert!(hash.starts_with("sha256:"));
        assert!(verify_replay_hash(&hash, &input, &rules, &comp).unwrap());
    }

    #[test]
    fn test_modification_detection() {
        let input = json!({"state": "PENDING_VERIFICATION"});
        let rules = json!({"version": "0.2.0"});
        let comp = json!({"steps": [{"op": "check"}]});

        let hash = compute_replay_hash(&input, &rules, &comp).unwrap();

        // Modify input
        let modified_input = json!({"state": "SETTLED"});
        assert!(!verify_replay_hash(&hash, &modified_input, &rules, &comp).unwrap());
    }

    #[test]
    fn test_deterministic() {
        let input = json!({"a": 1});
        let rules = json!({"b": 2});
        let comp = json!({"c": 3});

        let hash1 = compute_replay_hash(&input, &rules, &comp).unwrap();
        let hash2 = compute_replay_hash(&input, &rules, &comp).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_key_order_independent() {
        let input1 = json!({"z": 1, "a": 2});
        let input2 = json!({"a": 2, "z": 1});
        let rules = json!({"v": "0.2.0"});
        let comp = json!({"s": []});

        let hash1 = compute_replay_hash(&input1, &rules, &comp).unwrap();
        let hash2 = compute_replay_hash(&input2, &rules, &comp).unwrap();
        assert_eq!(hash1, hash2);
    }
}
