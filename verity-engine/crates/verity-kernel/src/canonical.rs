use serde::Serialize;
use sha2::{Sha256, Digest};
use crate::VerityError;

/// Canonical JSON serialization: sorted keys, no whitespace, UTF-8.
/// This is critical for hash computation — the same object must always
/// produce the same byte sequence.
pub fn canonical_serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, VerityError> {
    let json_value = serde_json::to_value(value)
        .map_err(|e| VerityError::SerializationError(e.to_string()))?;
    let sorted = sort_json_value(&json_value);
    serde_json::to_vec(&sorted)
        .map_err(|e| VerityError::SerializationError(e.to_string()))
}

/// SHA-256 hash of canonical serialization, prefixed with "sha256:"
pub fn canonical_hash<T: Serialize>(value: &T) -> Result<String, VerityError> {
    let bytes = canonical_serialize(value)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hasher.finalize();
    Ok(format!("sha256:{}", hex::encode(hash)))
}

/// Recursively sort JSON object keys for deterministic serialization.
fn sort_json_value(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted.insert(key.clone(), sort_json_value(&map[key]));
            }
            serde_json::Value::Object(sorted)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(sort_json_value).collect())
        }
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deterministic_serialization() {
        let val = json!({"b": 2, "a": 1, "c": 3});
        let bytes1 = canonical_serialize(&val).unwrap();
        let bytes2 = canonical_serialize(&val).unwrap();
        assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn test_keys_sorted() {
        let val = json!({"z": 1, "a": 2, "m": 3});
        let bytes = canonical_serialize(&val).unwrap();
        let s = String::from_utf8(bytes).unwrap();
        assert_eq!(s, r#"{"a":2,"m":3,"z":1}"#);
    }

    #[test]
    fn test_nested_keys_sorted() {
        let val = json!({"b": {"z": 1, "a": 2}, "a": 1});
        let bytes = canonical_serialize(&val).unwrap();
        let s = String::from_utf8(bytes).unwrap();
        assert_eq!(s, r#"{"a":1,"b":{"a":2,"z":1}}"#);
    }

    #[test]
    fn test_hash_prefixed() {
        let val = json!({"test": true});
        let hash = canonical_hash(&val).unwrap();
        assert!(hash.starts_with("sha256:"));
        assert_eq!(hash.len(), 7 + 64); // "sha256:" + 64 hex chars
    }

    #[test]
    fn test_hash_deterministic() {
        let val = json!({"b": 2, "a": 1});
        let hash1 = canonical_hash(&val).unwrap();
        let hash2 = canonical_hash(&val).unwrap();
        assert_eq!(hash1, hash2);
    }
}
