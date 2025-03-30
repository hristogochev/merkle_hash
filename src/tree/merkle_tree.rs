use crate::components::merkle_item::MerkleItem;
use crate::iters::merkle_node_into_iter::MerkleNodeIntoIter;
use crate::iters::merkle_node_iter::MerkleNodeIter;
use crate::tree::merkle_node::MerkleNode;
use crate::tree::merkle_tree_builder::MerkleTreeBuilder;
use crate::utils::algorithm::Algorithm;

/// Represents an indexed directory tree
#[cfg_attr(feature = "bincode", derive(bincode::Decode, bincode::Encode))]
pub struct MerkleTree {
    pub root: MerkleNode,
}

impl MerkleTree {
    /// Creates a new merkle tree builder
    ///
    /// - Default hash_names is **false**
    /// - Default algorithm is **blake3**
    pub fn builder(root_absolute_path: impl AsRef<str>) -> MerkleTreeBuilder {
        let absolute_root_path = root_absolute_path.as_ref().to_owned();
        MerkleTreeBuilder {
            absolute_root_path,
            hash_names: false,
            algorithm: Algorithm::default(),
        }
    }
    /// Returns an iterator over each file and directory in the tree
    pub fn iter(&self) -> MerkleNodeIter {
        self.root.iter()
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
        self.root.into_iter()
    }
}
