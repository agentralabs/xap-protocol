use serde::Serialize;
use verity_kernel::{VerityId, SettlementId, VerityError, canonical_hash};

/// A chain of VerityReceipts for a single settlement.
/// Each receipt's hash includes the previous receipt's hash.
/// Invariant 8: Truth history is append-only.
pub struct VerityChain {
    settlement_id: SettlementId,
    entries: Vec<ChainEntry>,
}

#[derive(Debug, Clone)]
pub struct ChainEntry {
    pub position: u32,
    pub verity_id: VerityId,
    pub content_hash: String,
    pub previous_hash: Option<String>,
    pub chain_hash: String,
}

impl VerityChain {
    pub fn new(settlement_id: SettlementId) -> Self {
        Self {
            settlement_id,
            entries: Vec::new(),
        }
    }

    /// Append a new entry to the chain. Returns the chain_hash.
    pub fn append(
        &mut self,
        verity_id: VerityId,
        content: &impl Serialize,
    ) -> Result<String, VerityError> {
        let content_hash = canonical_hash(content)?;
        let previous_hash = self.entries.last().map(|e| e.chain_hash.clone());
        let position = (self.entries.len() as u32) + 1;

        let chain_input = ChainHashInput {
            content_hash: &content_hash,
            previous_hash: previous_hash.as_deref(),
        };
        let chain_hash = canonical_hash(&chain_input)?;

        let entry = ChainEntry {
            position,
            verity_id,
            content_hash,
            previous_hash,
            chain_hash: chain_hash.clone(),
        };
        self.entries.push(entry);
        Ok(chain_hash)
    }

    /// Verify the entire chain is intact (no gaps, no tampering).
    pub fn verify(&self) -> Result<bool, VerityError> {
        for (i, entry) in self.entries.iter().enumerate() {
            // Check position is sequential
            if entry.position != (i as u32) + 1 {
                return Ok(false);
            }

            // Check previous_hash linkage
            if i == 0 {
                if entry.previous_hash.is_some() {
                    return Ok(false);
                }
            } else if entry.previous_hash.as_ref() != Some(&self.entries[i - 1].chain_hash) {
                return Ok(false);
            }

            // Recompute chain_hash and verify
            let chain_input = ChainHashInput {
                content_hash: &entry.content_hash,
                previous_hash: entry.previous_hash.as_deref(),
            };
            let expected = canonical_hash(&chain_input)?;
            if entry.chain_hash != expected {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn latest_hash(&self) -> Option<&str> {
        self.entries.last().map(|e| e.chain_hash.as_str())
    }

    pub fn settlement_id(&self) -> &SettlementId {
        &self.settlement_id
    }

    pub fn entries(&self) -> &[ChainEntry] {
        &self.entries
    }
}

#[derive(Serialize)]
struct ChainHashInput<'a> {
    content_hash: &'a str,
    previous_hash: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_settlement_id() -> SettlementId {
        SettlementId::new("stl_4b7c9e2f").unwrap()
    }

    #[test]
    fn test_append_and_verify() {
        let mut chain = VerityChain::new(test_settlement_id());
        let v1 = VerityId::new("vrt_a1b2c3d4").unwrap();
        let v2 = VerityId::new("vrt_e5f6a7b8").unwrap();

        chain.append(v1, &json!({"decision": "release"})).unwrap();
        chain.append(v2, &json!({"decision": "split"})).unwrap();

        assert_eq!(chain.len(), 2);
        assert!(chain.verify().unwrap());
    }

    #[test]
    fn test_first_entry_no_previous() {
        let mut chain = VerityChain::new(test_settlement_id());
        let v1 = VerityId::new("vrt_a1b2c3d4").unwrap();
        chain.append(v1, &json!({"test": true})).unwrap();

        assert!(chain.entries()[0].previous_hash.is_none());
    }

    #[test]
    fn test_subsequent_entries_link() {
        let mut chain = VerityChain::new(test_settlement_id());
        let v1 = VerityId::new("vrt_a1b2c3d4").unwrap();
        let v2 = VerityId::new("vrt_e5f6a7b8").unwrap();

        chain.append(v1, &json!({"step": 1})).unwrap();
        chain.append(v2, &json!({"step": 2})).unwrap();

        assert_eq!(
            chain.entries()[1].previous_hash.as_ref().unwrap(),
            &chain.entries()[0].chain_hash
        );
    }

    #[test]
    fn test_tamper_detection() {
        let mut chain = VerityChain::new(test_settlement_id());
        let v1 = VerityId::new("vrt_a1b2c3d4").unwrap();
        let v2 = VerityId::new("vrt_e5f6a7b8").unwrap();

        chain.append(v1, &json!({"step": 1})).unwrap();
        chain.append(v2, &json!({"step": 2})).unwrap();

        // Tamper with first entry's content_hash
        chain.entries[0].content_hash = "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string();

        assert!(!chain.verify().unwrap());
    }

    #[test]
    fn test_latest_hash() {
        let mut chain = VerityChain::new(test_settlement_id());
        assert!(chain.latest_hash().is_none());

        let v1 = VerityId::new("vrt_a1b2c3d4").unwrap();
        let hash = chain.append(v1, &json!({"test": true})).unwrap();
        assert_eq!(chain.latest_hash().unwrap(), hash);
    }
}
