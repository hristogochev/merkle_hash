use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::components::merkle_item::MerkleItem;
use crate::components::merkle_path::MerklePath;
use crate::error::IndexingError;
use crate::utils::algorithm::Algorithm;

/// Represents a single node on the merkle tree
#[derive(Eq, PartialEq, Debug, Clone)]
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub struct MerkleNode {
    pub item: MerkleItem,
    pub children: BTreeSet<MerkleNode>,
}

impl PartialOrd<Self> for MerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl MerkleNode {
    /// Creates a new root node
    pub fn root(root: &str, hash_names: bool, algorithm: Algorithm) -> Result<Self, IndexingError> {
        // Creates a new empty relative path, as this is the root
        #[cfg(not(feature = "camino"))]
        let relative_path = std::path::PathBuf::from("");
        #[cfg(feature = "camino")]
        let relative_path = camino::Utf8PathBuf::from("");

        // Gets an owned copy of the absolute path
        #[cfg(not(feature = "camino"))]
        let absolute_path = std::path::PathBuf::from(root);
        #[cfg(feature = "camino")]
        let absolute_path = camino::Utf8PathBuf::from(root);

        // Creates a new merkle path based on them both
        let path = MerklePath::new(relative_path, absolute_path);

        // Indexes the newly created node and returns the result
        Self::index(root, path, hash_names, &algorithm)
    }

    /// Indexes a new node, finding its relative and absolute paths, its file/directory hash
    /// and the same for all of its descendants
    fn index(
        root: &str,
        path: MerklePath,
        hash_names: bool,
        algorithm: &Algorithm,
    ) -> Result<MerkleNode, IndexingError> {
        // Indexes its direct descendants for their hashes and paths
        let children = if path.absolute.is_dir() {
            let read_dir = match fs::read_dir(&path.absolute) {
                Ok(ok) => ok,
                Err(err) => return Err(IndexingError::UnableToReadDir(path.absolute, err)),
            };

            #[cfg(feature = "parallel")]
            let read_dir = read_dir.par_bridge();

            read_dir
                .map(|entry| {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            return Err(IndexingError::UnableToReadDirEntry(
                                path.absolute.clone(),
                                err,
                            ))
                        }
                    };

                    #[cfg(not(feature = "camino"))]
                    let absolute_path = entry.path();

                    #[cfg(feature = "camino")]
                    let absolute_path = camino::Utf8PathBuf::from_path_buf(entry.path())
                        .map_err(|path| IndexingError::PathIsNotValidUtf8(path))?;

                    let relative_path = match absolute_path.strip_prefix(root) {
                        Ok(relative_path) => relative_path.to_path_buf(),
                        Err(err) => {
                            return Err(IndexingError::UnableToStripRootPrefix(
                                absolute_path,
                                root.to_string(),
                                err,
                            ))
                        }
                    };

                    let path = MerklePath::new(relative_path, absolute_path);

                    let node = Self::index(root, path, hash_names, algorithm)?;

                    Ok(node)
                })
                .collect::<Result<BTreeSet<MerkleNode>, IndexingError>>()?
        } else {
            BTreeSet::new()
        };

        // Finds the node's contents hash
        let contents_hash: Vec<u8> = if path.absolute.is_dir() {
            let hashes: Vec<_> = children
                .iter()
                .map(|child| child.item.hash.as_slice())
                .collect();

            match algorithm.compute_merkle_hash(&hashes) {
                Some(hash) => hash,
                None => algorithm.compute_hash(b""),
            }
        } else {
            let file_bytes = match fs::read(&path.absolute) {
                Ok(file_bytes) => file_bytes,
                Err(err) => return Err(IndexingError::UnableToReadFile(path.absolute, err)),
            };

            algorithm.compute_hash(&file_bytes)
        };

        // Check if names should be included in the hashing results and get the output hash
        let hash: Vec<u8> = if hash_names {
            // Gets the node path's name
            let name = match path.absolute.file_name() {
                None => return Err(IndexingError::UnableToReadFileName(path.absolute)),
                Some(name) => name,
            };

            #[cfg(not(feature = "camino"))]
            let name = match name.to_str() {
                None => return Err(IndexingError::UnableToReadFileName(path.absolute)),
                Some(name) => name,
            };

            // Create a hashing stack
            algorithm.compute_hash_from_slices(name.as_bytes(), &contents_hash)
        } else {
            contents_hash
        };

        #[cfg(feature = "retain")]
        // Get the direct descendant paths
        let children_paths = Self::get_children_paths(&children);

        // Returns the newly created node with its data

        #[cfg(feature = "retain")]
        let item = MerkleItem::new(path, hash, children_paths);
        #[cfg(not(feature = "retain"))]
        let item = MerkleItem::new(path, hash);

        let node = MerkleNode { item, children };

        Ok(node)
    }

    #[cfg(feature = "retain")]
    fn get_children_paths(children: &BTreeSet<MerkleNode>) -> BTreeSet<MerklePath> {
        #[cfg(feature = "parallel")]
        let children_iter = children.par_iter();

        #[cfg(not(feature = "parallel"))]
        let children_iter = children.iter();

        children_iter.map(|child| child.item.path.clone()).collect()
    }
}
