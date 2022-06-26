use camino::Utf8PathBuf;
use std::cmp::Ordering;

/// An utility struct containing an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerklePath {
    pub relative_path: Utf8PathBuf,
    pub absolute_path: Utf8PathBuf,
}

impl MerklePath {
    pub fn new(relative_path: Utf8PathBuf, absolute_path: Utf8PathBuf) -> Self {
        Self {
            relative_path,
            absolute_path,
        }
    }
}

impl PartialOrd<Self> for MerklePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relative_path.partial_cmp(&other.relative_path)
    }
}

impl Ord for MerklePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative_path.cmp(&other.relative_path)
    }
}
