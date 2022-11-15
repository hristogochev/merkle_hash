use crate::merkle_path::MerklePath;
use crate::merkle_utils::compute_merkle_hash;
use anyhow::{Context, Result};
use blake3::{hash, Hash, Hasher};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};

use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Merkle node struct that consists of a path, a hash and the children nodes relative to it
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MerkleNode {
    pub path: MerklePath,
    pub hash: Hash,
    pub children: BTreeSet<MerkleNode>,
}

impl PartialOrd<Self> for MerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for MerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl MerkleNode {
    /// Creates a new merkle node, its paths, hashes and the same for all of its descendants
    pub fn new(absolute_root: impl AsRef<Path>) -> Result<Self> {
        // Creates a new empty relative path, as this is the root
        let relative_path = Utf8PathBuf::from("");

        // Gets an owned copy of the absolute path
        let absolute_path = Utf8Path::from_path(absolute_root.as_ref())
            .with_context(|| "Path is not valid UTF8 path")?
            .to_path_buf();

        // Creates a new merkle path based on them both
        let path = MerklePath::new(relative_path, absolute_path);

        // Indexes the newly created node and returns the result
        Self::get_node(absolute_root, path)
    }
    /// Indexes a new node, finding its relative path, its hash based on its contents and name
    /// and all the same for all of its descendants
    fn get_node(root: impl AsRef<Path>, path: MerklePath) -> Result<MerkleNode> {
        // Creates an owned copy of the root path
        let root = root.as_ref().to_path_buf();

        // Indexes its direct descendants for their hashes and paths
        let children: BTreeSet<MerkleNode> = if path.absolute_path.is_dir() {
            let children: Result<BTreeSet<MerkleNode>> = WalkDir::new(&path.absolute_path)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .par_bridge()
                .map(|entry| {
                    let absolute_path = Utf8Path::from_path(entry?.path())
                        .with_context(|| "Path is not valid UTF8 path")?
                        .to_path_buf();
                    let relative_path = absolute_path.strip_prefix(&root)?.to_path_buf();
                    let merkle_path = MerklePath::new(relative_path, absolute_path);
                    let merkle_node = Self::get_node(&root, merkle_path)?;
                    Ok(merkle_node)
                })
                .collect();
            children?
        } else {
            BTreeSet::new()
        };

        // Gets the node path's name
        let name = path
            .absolute_path
            .file_name()
            .with_context(|| format!("File name missing for: {}", path.absolute_path))?;

        // Finds the node's contents hash
        let contents_hash: Option<Hash> = if path.absolute_path.is_dir() {
            let hashes: Vec<Hash> = children.iter().map(|child| child.hash).collect();
            compute_merkle_hash(&hashes)
        } else {
            let file_bytes = fs::read(&path.absolute_path)?;
            Some(hash(&file_bytes))
        };

        // Combines the node name hash and the contents hash
        let mut hasher = Hasher::new();
        hasher.update(name.as_bytes());
        if let Some(contents_hash) = contents_hash {
            hasher.update(contents_hash.as_bytes());
        }

        // Returns the newly created node with its data
        let hash = hasher.finalize();
        let node = MerkleNode {
            path,
            hash,
            children,
        };

        Ok(node)
    }

    /// Traverses the node, executing an action for itself and its children
    pub fn traverse<N>(&self, on_node: &N) -> Result<()>
    where
        N: Fn(&MerklePath, &Hash) -> Result<()>,
    {
        on_node(&self.path, &self.hash)?;
        self.children
            .iter()
            .try_for_each(|child| child.traverse(on_node))
    }

    /// Traverses the node, executing an action for itself and its children on multiple threads
    pub fn traverse_par<N>(&self, on_node: &N) -> Result<()>
    where
        N: Fn(&MerklePath, &Hash) -> Result<()> + Sync + Send,
    {
        on_node(&self.path, &self.hash)?;
        self.children
            .par_iter()
            .try_for_each(|child| child.traverse_par(on_node))
    }

    /// Collapses the node its children's contents into a BTreeSet
    /// Use this if you DO care about the order of nodes
    pub fn collapse_into_tree_set(self, output_set: &mut BTreeSet<CollapsedMerkleNode>) {
        let children_names: BTreeSet<String> = self
            .children
            .par_iter()
            .map(|child| child.path.relative_path.to_string())
            .collect();
        let collapsed_merkle_node = CollapsedMerkleNode::new(self.path, self.hash, children_names);
        output_set.insert(collapsed_merkle_node);
        self.children
            .into_iter()
            .for_each(|child| child.collapse_into_tree_set(output_set));
    }

    /// Collapses the node its children's contents into a HashSet
    /// Use this if you DO NOT care about the order of nodes
    pub fn collapse_into_hashset(self, output_set: &mut HashSet<CollapsedMerkleNode>) {
        let children_names: BTreeSet<String> = self
            .children
            .par_iter()
            .map(|child| child.path.relative_path.to_string())
            .collect();
        let collapsed_merkle_node = CollapsedMerkleNode::new(self.path, self.hash, children_names);
        output_set.insert(collapsed_merkle_node);
        self.children
            .into_iter()
            .for_each(|child| child.collapse_into_hashset(output_set));
    }
}

/// Merkle node from a merkle tree that has been collapsed into a set
#[derive(Eq, PartialEq, Hash)]
pub struct CollapsedMerkleNode {
    pub path: MerklePath,
    pub hash: Hash,
    pub children: BTreeSet<String>,
}

impl CollapsedMerkleNode {
    pub fn new(path: MerklePath, hash: Hash, children: BTreeSet<String>) -> Self {
        Self {
            path,
            hash,
            children,
        }
    }
}

impl PartialOrd<Self> for CollapsedMerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.path.partial_cmp(&other.path)
    }
}

impl Ord for CollapsedMerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
