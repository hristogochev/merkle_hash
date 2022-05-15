use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct MerklePath {
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
}

impl MerklePath {
    pub fn new(relative_path: PathBuf, absolute_path: PathBuf) -> Self {
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
