/*!
Crate `merkle_hash` makes it easy to find the hashes of all files and directories in a directory tree.

# Features

* Finds the master hash of a directory tree with ease.
* External iteration over the paths and hashes of each file and directory.
* Ability to specify whether names should be included in the hashes of files and directories.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3"
```

# Example: Get the master hash of a directory tree:
```
use merkle_hash::MerkleTree;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
let master_hash = tree.main_node.item.hash;
```

# Example: Iterate over a directory tree, getting the hash of each file and directory:
```
use merkle_hash::MerkleTree;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
for item in tree {
    println!("{}: {}", item.path.relative, item.hash);
}
```

# Example: Collapse the tree into any linear collection:
```
use std::collections::BTreeSet;
use merkle_hash::MerkleTree;
use merkle_hash::MerkleItem;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
let btree_set: BTreeSet<MerkleItem> = tree.into_iter().collect();
```
 */

/// Holds the path, hash and children paths of a file or directory
pub mod merkle_item;
/// Represents a single node on the merkle tree
pub mod merkle_node;
/// Owned node iterator
pub mod merkle_node_into_iter;
/// Node iterator
pub mod merkle_node_iter;
/// Utility that represents relative and absolute paths
pub mod merkle_path;
/// The main tree with merkle hashes
pub mod merkle_tree;
/// Merkle hashing utility functions
pub mod merkle_utils;

/// Merkle item reexport
pub use merkle_item::MerkleItem;
/// Merkle tree reexport
pub use merkle_tree::MerkleTree;

/// Used dependencies reexport
pub use anyhow;
pub use blake3;
pub use camino;
pub use rayon;
