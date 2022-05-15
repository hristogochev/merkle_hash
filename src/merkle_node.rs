use blake3::Hash;
use std::cmp::Ordering;
use std::path::PathBuf;

/// Merkle node struct for usage in a merkle tree
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MerkleNode {
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
    pub hash: Hash,
}

impl MerkleNode {
    pub fn new(relative_path: PathBuf, absolute_path: PathBuf, hash: Hash) -> Self {
        Self {
            relative_path,
            absolute_path,
            hash,
        }
    }
}

impl PartialOrd<Self> for MerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relative_path.partial_cmp(&other.relative_path)
    }
}

impl Ord for MerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative_path.cmp(&other.relative_path)
    }
}
