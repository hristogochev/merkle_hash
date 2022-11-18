use crate::merkle_path::MerklePath;
use blake3::Hash;
use std::cmp::Ordering;

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerkleItem {
    pub path: MerklePath,
    pub hash: Hash,
    pub children_paths: Vec<MerklePath>,
}

impl MerkleItem {
    pub fn new(path: MerklePath, hash: Hash, children_paths: Vec<MerklePath>) -> Self {
        Self {
            path,
            hash,
            children_paths,
        }
    }
}

impl PartialOrd<Self> for MerkleItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for MerkleItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
