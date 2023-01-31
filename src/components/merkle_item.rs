use crate::components::merkle_path::MerklePath;
use std::cmp::Ordering;

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerkleItem {
    pub path: MerklePath,
    pub hash: [u8; 32],
    pub children_paths: Vec<MerklePath>,
}

impl MerkleItem {
    pub fn new(path: MerklePath, hash: [u8; 32], children_paths: Vec<MerklePath>) -> Self {
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
