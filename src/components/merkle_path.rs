#[cfg(feature = "camino")]
use camino::Utf8PathBuf;
use std::cmp::Ordering;
#[cfg(not(feature = "camino"))]
use std::path::PathBuf;

/// An utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerklePath {
    #[cfg(feature = "camino")]
    pub relative: Utf8PathBuf,
    #[cfg(feature = "camino")]
    pub absolute: Utf8PathBuf,
    #[cfg(not(feature = "camino"))]
    pub relative: PathBuf,
    #[cfg(not(feature = "camino"))]
    pub absolute: PathBuf,
}

impl MerklePath {
    #[cfg(feature = "camino")]
    pub fn new(relative_path: Utf8PathBuf, absolute_path: Utf8PathBuf) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
        }
    }
    #[cfg(not(feature = "camino"))]
    pub fn new(relative_path: PathBuf, absolute_path: PathBuf) -> Self {
        Self {
            relative: relative_path,
            absolute: absolute_path,
        }
    }
}

impl PartialOrd<Self> for MerklePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relative.partial_cmp(&other.relative)
    }
}

impl Ord for MerklePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative.cmp(&other.relative)
    }
}
