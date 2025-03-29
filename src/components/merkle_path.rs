use std::cmp::Ordering;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};
use camino::Utf8PathBuf;

/// A utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
pub struct MerklePath {
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub relative: Utf8PathBuf,
    
    #[cfg_attr(feature = "bincode", bincode(with_serde))]
    pub absolute: Utf8PathBuf,
}

impl MerklePath {
    pub fn new(relative_path: Utf8PathBuf, absolute_path: Utf8PathBuf) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
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
