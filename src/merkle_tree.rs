use crate::merkle_node::{CollapsedMerkleNode, MerkleNode};
use crate::merkle_path::MerklePath;
use anyhow::Result;
use blake3::Hash;
use std::collections::{BTreeSet, HashSet};
use std::path::Path;

/// Merkle tree struct that contains a node at its root
pub struct MerkleTree {
    main_node: MerkleNode,
}

impl MerkleTree {
    /// Creates a new tree indexing its descendants upon creation
    pub fn new(absolute_path: impl AsRef<Path>) -> Result<Self> {
        let main_node = MerkleNode::new(absolute_path)?;
        Ok(Self { main_node })
    }

    /// Traverses the tree, executing an action for each node in it in consequence
    pub fn traverse<N>(&self, on_node: &N) -> Result<()>
    where
        N: Fn(&MerklePath, &Hash) -> Result<()>,
    {
        self.main_node.traverse(on_node)
    }

    /// Traverses the tree, executing an action for each node in it on multiple threads,
    /// the execution of the action for each node is not consequential
    pub fn traverse_par<N>(&self, on_node: &N) -> Result<()>
    where
        N: Fn(&MerklePath, &Hash) -> Result<()> + Sync + Send,
    {
        self.main_node.traverse_par(on_node)
    }

    /// Collapses the tree's contents into a BTreeSet of collapsed merkle nodes
    /// Use this if you DO care about the order of nodes based on their paths
    pub fn collapse_into_tree_set(self) -> BTreeSet<CollapsedMerkleNode> {
        let mut set = BTreeSet::new();
        self.main_node.collapse_into_tree_set(&mut set);
        set
    }

    /// Collapses the tree's contents into a HashSet of collapsed merkle nodes
    /// Use this if you DO NOT care about the order of nodes based on their paths
    pub fn collapse_into_hash_set(self) -> HashSet<CollapsedMerkleNode> {
        let mut set = HashSet::new();
        self.main_node.collapse_into_hashset(&mut set);
        set
    }
}
