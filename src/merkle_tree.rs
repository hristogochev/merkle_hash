use crate::merkle_item::MerkleItem;
use crate::merkle_node::MerkleNode;
use crate::merkle_node_into_iter::MerkleNodeIntoIter;
use crate::merkle_node_iter::MerkleNodeIter;
use anyhow::Result;
use std::path::Path;

/// Represents an indexed directory tree
pub struct MerkleTree {
    pub main_node: MerkleNode,
}

impl MerkleTree {
    /// Creates a new tree indexing its descendants upon creation
    /// - If hash_names is true, includes the names of the files and directories in their hashes.
    pub fn new(root_absolute_path: impl AsRef<Path>, hash_names: bool) -> Result<Self> {
        let main_node = MerkleNode::new(root_absolute_path, hash_names)?;
        Ok(Self { main_node })
    }

    /// Returns an iterator over each file and directory in the tree
    pub fn iter(&self) -> MerkleNodeIter {
        self.main_node.iter()
    }
}

impl<'a> IntoIterator for &'a MerkleTree {
    type Item = &'a MerkleItem;

    type IntoIter = MerkleNodeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for MerkleTree {
    type Item = MerkleItem;

    type IntoIter = MerkleNodeIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.main_node.into_iter()
    }
}
