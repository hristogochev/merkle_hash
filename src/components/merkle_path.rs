use camino::Utf8PathBuf;
use std::cmp::Ordering;

/// An utility struct that contains an absolute path and a relative path
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct MerklePath {
    pub relative: Utf8PathBuf,
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
        self.relative.partial_cmp(&other.relative)
    }
}

impl Ord for MerklePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.relative.cmp(&other.relative)
    }
}
