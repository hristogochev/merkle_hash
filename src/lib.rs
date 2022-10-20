/*!
Crate `merkle_hash` makes it easy to find the hashes of all files and directories in a directory tree.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "2"

```
# Example: Get the master hash of a directory tree:
```
use merkle_hash::merkle_tree::MerkleTree;

let tree = MerkleTree::new("/path/to/tree").unwrap();
let master_hash = tree.main_node.hash;
```

# Example: Traverse a directory tree, getting the hash of each file and directory:
```
use merkle_hash::merkle_tree::MerkleTree;

let tree=MerkleTree::new("/path/to/tree").unwrap();
let traverse_result = tree.traverse(&|path,hash|{
    println!("{}: {}", path.absolute_path, hash);
    Ok(())
});
```

# Example: Collapse the tree for linear traversal:
```
use merkle_hash::merkle_tree::MerkleTree;

let tree = MerkleTree::new("/path/to/tree").unwrap();
let btree_set = tree.collapse_into_tree_set();
 */

/// Represents a single node on the merkle tree
pub mod merkle_node;
/// Utility that represents relative and absolute paths
pub mod merkle_path;
/// The main tree with merkle hashes
pub mod merkle_tree;
/// Merkle hashing utility functions
pub mod merkle_utils;

pub use blake3;
