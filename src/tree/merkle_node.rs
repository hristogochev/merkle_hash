use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::components::merkle_item::MerkleItem;
use crate::components::merkle_path::MerklePath;
use crate::utils::algorithm::{Algorithm, MerkleHashAlgorithm};

/// Represents a single node on the merkle tree
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MerkleNode<const N: usize> {
    pub item: MerkleItem<N>,
    pub children: BTreeSet<MerkleNode<N>>,
}

impl<const N: usize> PartialOrd<Self> for MerkleNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

impl<const N: usize> Ord for MerkleNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl<const N: usize> MerkleNode<N> {
    /// Creates a new root node
    pub fn root<A: Algorithm<N>>(root: &str, hash_names: bool, algorithm: A) -> Result<Self> {
        // Creates a new empty relative path, as this is the root
        let relative_path = Utf8PathBuf::from("");

        // Gets an owned copy of the absolute path
        let absolute_path = Utf8PathBuf::from(root);

        // Creates a new merkle path based on them both
        let path = MerklePath::new(relative_path, absolute_path);

        // Indexes the newly created node and returns the result
        Self::index(root, path, hash_names, &algorithm)
    }

    /// Indexes a new node, finding its relative and absolute paths, its file/directory hash
    /// and the same for all of its descendants
    fn index<A: Algorithm<N>>(
        root: &str,
        path: MerklePath,
        hash_names: bool,
        algorithm: &A,
    ) -> Result<MerkleNode<N>> {
        // Indexes its direct descendants for their hashes and paths
        let children = if path.absolute.is_dir() {
            let read_dir = fs::read_dir(&path.absolute)?;

            #[cfg(feature = "parallel")]
                let read_dir = read_dir.par_bridge();

            read_dir
                .map(|entry| {
                    let absolute_path = match Utf8PathBuf::from_path_buf(entry?.path()) {
                        Ok(absolute_path) => absolute_path,
                        Err(path) => bail!("Path is not valid UTF8 path: {}", path.display()),
                    };
                    let relative_path = absolute_path.strip_prefix(root)?.to_path_buf();
                    let path = MerklePath::new(relative_path, absolute_path);
                    let node = Self::index(root, path, hash_names, algorithm)?;
                    Ok(node)
                })
                .collect::<Result<BTreeSet<MerkleNode<N>>>>()?
        } else {
            BTreeSet::new()
        };

        // Finds the node's contents hash
        let contents_hash: [u8; N] = if path.absolute.is_dir() {
            let hashes: Vec<_> = children
                .iter()
                .map(|child| &child.item.hash)
                .collect();

            match algorithm.compute_merkle_hash(&hashes) {
                Some(hash) => hash,
                None => algorithm.compute_hash(b""),
            }
        } else {
            let file_bytes = fs::read(&path.absolute)
                .with_context(|| format!("Unable to read file: {}", path.absolute))?;

            algorithm.compute_hash(&file_bytes)
        };

        // Check if names should be included in the hashing results and get the output hash
        let hash: [u8; N] = if hash_names {
            // Gets the node path's name
            let name = path
                .absolute
                .file_name()
                .with_context(|| format!("Unable to read file: {}", path.absolute))?;

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
    fn get_children_paths(children: &BTreeSet<MerkleNode<N>>) -> BTreeSet<MerklePath> {
        #[cfg(feature = "parallel")]
            let children_iter = children.par_iter();

        #[cfg(not(feature = "parallel"))]
            let children_iter = children.iter();

        children_iter.map(|child| child.item.path.clone()).collect()
    }
}
