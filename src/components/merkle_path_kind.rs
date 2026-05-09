/// Represents the type of entry that a MerklePath points to (file, directory, or other)
#[cfg(feature = "kind")]
#[derive(Eq, PartialEq, Clone, Debug, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub enum MerklePathKind {
    File,
    Directory,
    Unknown
}

#[cfg(feature = "kind")]
impl MerklePathKind {
    /// Returns the entry kind of the supplied path
    pub fn from_path<T: AsRef<std::path::Path>>(path: &T) -> Self {
        let path = path.as_ref();
        if path.is_file() {
            Self::File
        } else if path.is_dir() {
            Self::Directory
        } else {
            Self::Unknown
        }
    }
}
