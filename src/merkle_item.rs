use crate::merkle_utils::get_merkle_hash;
use blake3::Hash;
use camino::{Utf8Path, Utf8PathBuf};
use walkdir::WalkDir;

/// Utility item for managing merkle hashes
pub struct MerkleItem {
    pub path: Utf8PathBuf,
}

impl MerkleItem {
    /// Creates a new merkle item from a given path
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let merkle_item = MerkleItem::new("/root/to/get/paths/from");
    /// ```
    pub fn new(path: impl AsRef<Utf8Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        MerkleItem { path }
    }

    /// Finds all direct descendants of the directory associated with the item
    ///
    /// - If the associated path is a path to a file, returns empty vector
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let merkle_item = MerkleItem::new("/root/to/get/paths/from");
    /// let children = merkle_item.get_children();
    /// ```
    pub fn get_children(&self) -> Vec<MerkleItem> {
        WalkDir::new(&self.path)
            .max_depth(1)
            .into_iter()
            .flatten()
            .filter(|entry| entry.path() != self.path)
            .flat_map(|entry| {
                let path = Utf8Path::from_path(entry.path())?;
                Some(MerkleItem::new(path))
            })
            .collect()
    }

    /// Finds the name of the file or directory associated with the item
    ///
    /// - If the file name cannot be resolved, returns None
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let merkle_item = MerkleItem::new("/root/to/get/paths/from");
    /// let name = merkle_item.get_name();
    /// ```
    pub fn get_name(&self) -> Option<&str> {
        self.path.file_name()
    }
    /// Finds the single merkle hash of all descendants of the item
    ///
    /// - If the path is invalid, returns None
    ///
    /// # Examples
    ///
    /// ```
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let merkle_item = MerkleItem::new("/root/to/get/paths/from");
    /// let merkle_hash = merkle_item.get_hash();
    /// ```
    pub fn get_hash(&self) -> Option<Hash> {
        get_merkle_hash(&self.path)
    }
}
