use crate::tree::merkle_node::MerkleNode;
use crate::utils::algorithm::Algorithm;
use crate::MerkleTree;
use anyhow::Result;

/// Utility builder pattern
pub struct MerkleTreeBuilder {
    /// Absolute root path of the tree
    pub(crate) absolute_root_path: String,
    /// Whether to include names in the hashes of files and directories, default is false
    pub(crate) hash_names: bool,
    /// Which hashing algorithm to use, default is blake3
    pub(crate) algorithm: Algorithm,
}

impl MerkleTreeBuilder {
    /// Sets whether to include the names of the files and directories in the hashing process, default is **false**
    pub fn hash_names(mut self, hash_names: bool) -> Self {
        self.hash_names = hash_names;
        self
    }

    /// Sets the hashing algorithm to use, default is **blake3**
    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Builds the hash tree by indexing all of its descendants
    pub fn build(self) -> Result<MerkleTree> {
        let root = MerkleNode::root(&self.absolute_root_path, self.hash_names, self.algorithm)?;
        Ok(MerkleTree { root })
    }
}
