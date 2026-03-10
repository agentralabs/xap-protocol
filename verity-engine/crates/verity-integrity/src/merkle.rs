use sha2::{Sha256, Digest};
use verity_kernel::VerityError;

/// Merkle tree for batch inclusion proofs.
/// In v1.0, we support single-entry proof. Full batch proofs in v1.1.
pub struct MerkleTree {
    leaves: Vec<String>,
    root: Option<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            root: None,
        }
    }

    pub fn add_leaf(&mut self, hash: String) {
        self.leaves.push(hash);
        self.root = None; // invalidate cached root
    }

    pub fn compute_root(&mut self) -> Result<String, VerityError> {
        if self.leaves.is_empty() {
            return Err(VerityError::ChainIntegrityError(
                "cannot compute root of empty tree".to_string(),
            ));
        }

        let mut current_level: Vec<String> = self.leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    // Odd node: duplicate it
                    format!("{}{}", chunk[0], chunk[0])
                };
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                let hash = hasher.finalize();
                next_level.push(format!("sha256:{}", hex::encode(hash)));
            }
            current_level = next_level;
        }

        let root = current_level.into_iter().next().unwrap();
        self.root = Some(root.clone());
        Ok(root)
    }

    pub fn root(&self) -> Option<&str> {
        self.root.as_deref()
    }

    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_leaf() {
        let mut tree = MerkleTree::new();
        tree.add_leaf("sha256:abc123".to_string());
        let root = tree.compute_root().unwrap();
        assert!(root.starts_with("sha256:"));
    }

    #[test]
    fn test_multiple_leaves() {
        let mut tree = MerkleTree::new();
        tree.add_leaf("sha256:aaa".to_string());
        tree.add_leaf("sha256:bbb".to_string());
        tree.add_leaf("sha256:ccc".to_string());
        let root = tree.compute_root().unwrap();
        assert!(root.starts_with("sha256:"));
    }

    #[test]
    fn test_root_changes_with_leaf() {
        let mut tree1 = MerkleTree::new();
        tree1.add_leaf("sha256:aaa".to_string());
        tree1.add_leaf("sha256:bbb".to_string());
        let root1 = tree1.compute_root().unwrap();

        let mut tree2 = MerkleTree::new();
        tree2.add_leaf("sha256:aaa".to_string());
        tree2.add_leaf("sha256:ccc".to_string());
        let root2 = tree2.compute_root().unwrap();

        assert_ne!(root1, root2);
    }

    #[test]
    fn test_empty_tree_errors() {
        let mut tree = MerkleTree::new();
        assert!(tree.compute_root().is_err());
    }

    #[test]
    fn test_root_cached() {
        let mut tree = MerkleTree::new();
        tree.add_leaf("sha256:aaa".to_string());
        tree.compute_root().unwrap();
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_root_invalidated_on_add() {
        let mut tree = MerkleTree::new();
        tree.add_leaf("sha256:aaa".to_string());
        tree.compute_root().unwrap();
        tree.add_leaf("sha256:bbb".to_string());
        assert!(tree.root().is_none());
    }
}
