use crate::merkle_node::MerkleNode;
use crate::merkle_utils::{find_merkle_hash_from_refs, get_hashes, get_paths};
use anyhow::Result;
use blake3::Hash;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Merkle tree implementation that stores the hashes of indexed files
/// and computes the hashes of directories
pub struct MerkleTree {
    pub root: PathBuf,
    pub relative_root: PathBuf,
    pub nodes: BTreeSet<MerkleNode>,
}

impl MerkleTree {
    /// Initializes the tree with a given root
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let relative_root = PathBuf::from("");
        let nodes = Self::index(&root)?;
        Ok(MerkleTree {
            root,
            relative_root,
            nodes,
        })
    }

    /// Indexes the paths and hashes of all descendants of the root
    fn index(root: impl AsRef<Path>) -> Result<BTreeSet<MerkleNode>> {
        let tree = get_paths(root)?;
        get_hashes(tree)
    }

    /// Computes the merkle hash for a given path relative to the root
    pub fn get_hash_for_path(&self, relative_path: impl AsRef<Path>) -> Option<Hash> {
        let relative_path = relative_path.as_ref();

        let hashes: Vec<&Hash> = self
            .nodes
            .par_iter()
            .filter(|node| node.relative_path.starts_with(relative_path))
            .map(|node| &node.hash)
            .collect();

        find_merkle_hash_from_refs(hashes)
    }
}
