use crate::components::merkle_item::MerkleItem;
use crate::iters::merkle_node_into_iter::MerkleNodeIntoIter;
use crate::iters::merkle_node_iter::MerkleNodeIter;
use crate::tree::merkle_node::MerkleNode;
use crate::tree::merkle_tree_builder::MerkleTreeBuilder;
use crate::utils::algorithm::Blake3;

/// Represents an indexed directory tree
pub struct MerkleTree<const N: usize> {
    pub root: MerkleNode<N>,
}

impl<const N: usize> MerkleTree<N> {
    /// Returns an iterator over each file and directory in the tree
    pub fn iter(&self) -> MerkleNodeIter<N> {
        self.root.iter()
    }
}

impl MerkleTree<32> {
    /// Creates a new merkle tree builder
    ///
    /// - Default hash_names is **false**
    /// - Default algorithm is **blake3**
    pub fn builder(root_absolute_path: impl AsRef<str>) -> MerkleTreeBuilder<32, Blake3> {
        let absolute_root_path = root_absolute_path.as_ref().to_owned();
        MerkleTreeBuilder {
            absolute_root_path,
            hash_names: false,
            algorithm: Blake3,
        }
    }
}

impl<'a, const N: usize> IntoIterator for &'a MerkleTree<N> {
    type Item = &'a MerkleItem<N>;

    type IntoIter = MerkleNodeIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize> IntoIterator for MerkleTree<N> {
    type Item = MerkleItem<N>;

    type IntoIter = MerkleNodeIntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
}
