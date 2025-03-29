use std::cmp::Ordering;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[cfg(feature = "retain")]
use std::collections::BTreeSet;

use crate::components::merkle_path::MerklePath;

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
pub struct MerkleItem {
    pub path: MerklePath,
    pub hash: Vec<u8>,
    #[cfg(feature = "retain")]
    pub children_paths: BTreeSet<MerklePath>,
}

impl MerkleItem {
    #[cfg(not(feature = "retain"))]
    pub fn new(path: MerklePath, hash: Vec<u8>) -> Self {
        Self {
            path,
            hash,
        }
    }
    #[cfg(feature = "retain")]
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
        Some(self.cmp(other))
    }
}

impl Ord for MerkleItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
