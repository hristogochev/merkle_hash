use std::cmp::Ordering;

#[cfg(feature = "kind")]
use crate::components::merkle_path_kind::MerklePathKind;

/// A utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub struct MerklePath {
    #[cfg(feature = "camino")]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub relative: camino::Utf8PathBuf,
    #[cfg(feature = "camino")]
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub absolute: camino::Utf8PathBuf,
    
    #[cfg(not(feature = "camino"))]
    pub relative: std::path::PathBuf,
    #[cfg(not(feature = "camino"))]
    pub absolute: std::path::PathBuf,

    #[cfg(feature = "kind")]
    pub kind: MerklePathKind
}

impl MerklePath {
    pub fn new(
        #[cfg(feature = "camino")]
        relative_path: camino::Utf8PathBuf,
        #[cfg(feature = "camino")]
        absolute_path: camino::Utf8PathBuf,

        #[cfg(not(feature = "camino"))]
        relative_path: std::path::PathBuf,
        #[cfg(not(feature = "camino"))]
        absolute_path: std::path::PathBuf,

        #[cfg(feature = "kind")]
        kind: MerklePathKind
    ) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
            #[cfg(feature = "kind")]
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
