use crate::components::merkle_path::MerklePath;
use std::cmp::Ordering;
#[cfg(feature = "retain")]
use std::collections::BTreeSet;

/// Holds the path, hash and children paths of a file or directory
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerkleItem<const N: usize> {
    pub path: MerklePath,
    pub hash: [u8; N],
    #[cfg(feature = "retain")]
    pub children_paths: BTreeSet<MerklePath>,
}

impl<const N: usize> MerkleItem<N> {
    #[cfg(not(feature = "retain"))]
    pub fn new(path: MerklePath, hash: [u8; N]) -> Self {
        Self {
            path,
            hash,
        }
    }
    #[cfg(feature = "retain")]
    pub fn new(path: MerklePath, hash: [u8; N], children_paths: BTreeSet<MerklePath>) -> Self {
        Self {
            path,
            hash,
            children_paths,
        }
    }
}

impl<const N: usize> PartialOrd<Self> for MerkleItem<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl<const N: usize> Ord for MerkleItem<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
