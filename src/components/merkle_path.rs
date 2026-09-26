use std::cmp::Ordering;

use crate::components::merkle_path_kind::MerklePathKind;

/// A utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerklePath {
    pub relative: std::path::PathBuf,
    pub absolute: std::path::PathBuf,
    pub parent_relative: Option<std::path::PathBuf>,
    pub kind: MerklePathKind
}

impl MerklePath {
    pub fn new(
        relative_path: std::path::PathBuf,
        absolute_path: std::path::PathBuf,
        parent_relative_path: Option<std::path::PathBuf>,
        kind: MerklePathKind
    ) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
            parent_relative: parent_relative_path,
            kind
        }
    }
}

impl PartialOrd<Self> for MerklePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MerklePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative.cmp(&other.relative)
    }
}
