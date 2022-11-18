use crate::merkle_path::MerklePath;
use crate::merkle_utils::compute_merkle_hash;
use anyhow::{bail, Context, Result};
use blake3::{hash, Hash, Hasher};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

use crate::merkle_item::MerkleItem;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::path::Path;

/// Merkle node struct that consists of the children nodes relative to it and an item with contents.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MerkleNode {
    pub item: MerkleItem,
    pub children: BTreeSet<MerkleNode>,
}

impl PartialOrd<Self> for MerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

impl Ord for MerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl MerkleNode {
    /// Creates a new merkle node
    pub fn new(absolute_root: impl AsRef<Path>, hash_names: bool) -> Result<Self> {
        // Creates a new empty relative path, as this is the root
        let relative_path = Utf8PathBuf::from("");

        // Gets an owned copy of the absolute path
        let absolute_path = Utf8Path::from_path(absolute_root.as_ref())
            .with_context(|| "Path is not valid UTF8 path")?
            .to_path_buf();

        // Creates a new merkle path based on them both
        let path = MerklePath::new(relative_path, absolute_path);

        // Indexes the newly created node and returns the result
        Self::get_node(absolute_root, path, hash_names)
    }

    /// Indexes a new node, finding its relative and absolute paths, its file/directory hash
    /// and the same for all of its descendants
    fn get_node(root: impl AsRef<Path>, path: MerklePath, hash_names: bool) -> Result<MerkleNode> {
        // Creates an owned copy of the root path
        let root = root.as_ref().to_path_buf();

        // Indexes its direct descendants for their hashes and paths
        let children: BTreeSet<MerkleNode> = if path.absolute.is_dir() {
            let children: Result<BTreeSet<MerkleNode>> = fs::read_dir(&path.absolute)?
                .par_bridge()
                .map(|entry| {
                    let Ok(absolute_path) = Utf8PathBuf::from_path_buf(entry?.path())else{
                        bail!("Path is not valid UTF8 path")
                    };
                    let relative_path = absolute_path.strip_prefix(&root)?.to_path_buf();
                    let merkle_path = MerklePath::new(relative_path, absolute_path);
                    let merkle_node = Self::get_node(&root, merkle_path, hash_names)?;
                    Ok(merkle_node)
                })
                .collect();

            children?
        } else {
            BTreeSet::new()
        };

        // Finds the node's contents hash
        let contents_hash: Hash = if path.absolute.is_dir() {
            let hashes: Vec<Hash> = children.iter().map(|child| child.item.hash).collect();
            match compute_merkle_hash(&hashes) {
                Some(hash) => hash,
                None => hash(b""),
            }
        } else {
            let file_bytes = fs::read(&path.absolute)?;
            hash(&file_bytes)
        };

        // Check if names should be included in the hashing results and get the output hash
        let hash: Hash = if hash_names {
            // Gets the node path's name
            let name = path
                .absolute
                .file_name()
                .with_context(|| format!("File name missing for: {}", path.absolute))?;

            // Create a hashing stack
            let mut hasher = Hasher::new();

            hasher.update(name.as_bytes());
            hasher.update(contents_hash.as_bytes());

            hasher.finalize()
        } else {
            contents_hash
        };

        // Get the direct descendant paths
        let children_paths = children
            .par_iter()
            .map(|child| child.item.path.clone())
            .collect();

        // Returns the newly created node with its data
        let item = MerkleItem::new(path, hash, children_paths);
        let node = MerkleNode { item, children };

        Ok(node)
    }
}
