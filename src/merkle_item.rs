use crate::merkle_utils::{find_merkle_hash, get_hashes, get_paths};
use blake3::Hash;
use std::ffi::OsStr;
use std::path::PathBuf;
use walkdir::WalkDir;

/// An item that simplifies the retrieval of hashes
pub struct MerkleItem {
    pub path: PathBuf,
}

impl MerkleItem {
    /// Creates a new merkle item from a given path
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let path = PathBuf::from("/root/to/get/paths/from");
    /// let merkle_item = MerkleItem::new(path);
    /// ```
    pub fn new(path: PathBuf) -> Self {
        MerkleItem { path }
    }

    /// Finds all direct descendants of the directory associated with the item
    ///
    /// - If the associated path is a path to a file, returns empty vector
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let path = PathBuf::from("/root/to/get/paths/from");
    /// let merkle_item = MerkleItem::new(path);
    /// let children = merkle_item.get_children();
    /// ```
    pub fn get_children(&self) -> Vec<MerkleItem> {
        WalkDir::new(&self.path)
            .max_depth(1)
            .into_iter()
            .flatten()
            .filter(|entry| entry.path() != self.path)
            .map(|entry| {
                let path = entry.path().to_path_buf();
                MerkleItem::new(path)
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
    /// use std::path::PathBuf;
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let path = PathBuf::from("/root/to/get/paths/from");
    /// let merkle_item = MerkleItem::new(path);
    /// let name = merkle_item.get_name();
    /// ```
    pub fn get_name(&self) -> Option<&str> {
        self.path.file_name().and_then(OsStr::to_str)
    }
    /// Finds the single merkle hash of all descendants of the item
    ///
    /// - If the path is invalid, returns None
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use merkle_hash::merkle_item::MerkleItem;
    ///
    /// let path = PathBuf::from("/root/to/get/paths/from");
    /// let merkle_item = MerkleItem::new(path);
    /// let merkle_hash = merkle_item.get_hash();
    /// ```
    pub fn get_hash(&self) -> Option<Hash> {
        let tree = get_paths(&self.path);
        let hashes = get_hashes(&tree);
        find_merkle_hash(&hashes)
    }
}
