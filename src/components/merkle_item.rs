use crate::components::merkle_path::MerklePath;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerkleItem {
    pub path: MerklePath,
    pub hash: Vec<u8>,
    pub children_paths: BTreeSet<MerklePath>,
}

impl MerkleItem {
    pub fn new(path: MerklePath, hash: Vec<u8>, children_paths: BTreeSet<MerklePath>) -> Self {
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
