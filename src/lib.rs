/*!
Crate `merkle_hash` makes it easy to find the hashes of all files and directories in a directory tree.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3.6"
```

# Features

* Finds the master hash of a directory tree with ease.
* Offers multiple hashing algorithms.
* Allows including names in the hashing process.
* Uses a merkle tree algorithm to compute the hashes of directories.
* External iteration over the paths and hashes of files and directories.

# Limitations

* Currently only supports UTF-8 paths and will fail if a path is not UTF-8 encoded.

# Optional

* `sha` - Add this cargo feature to include `SHA-256` and `SHA-512` as hashing algorithms.
* `parallel` - Enabled by default, this feature makes the crate utilize all available threads.
* `encode` - Enabled by default, this feature adds the `bytes_to_hex` and `to_hex_string` functions.
* `retain` - Disabled by default, this feature duplicates the children paths of directories upon traversal.

# Example: Get the master hash of a directory tree:
```
use merkle_hash::{algorithm::Blake3, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory")
    .algorithm(Blake3)
    .hash_names(false)
    .build()?;
let master_hash = tree.root.item.hash;
```

# Example: Iterate over a directory tree, getting the hash of each file and directory:
```
use merkle_hash::{Encodable, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
for item in tree {
    println!("{}: {}", item.path.relative, item.hash.to_hex_string());
}
```

# Example: Collapse the tree into any linear collection:
```
use std::collections::BTreeSet;
use merkle_hash::{MerkleItem, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
let btree_set: BTreeSet<_> = tree.into_iter().collect();
```
 */

/// Used dependencies reexport
pub use anyhow;
pub use blake3;
pub use camino;
#[cfg(feature = "parallel")]
pub use rayon;

pub use components::merkle_item::MerkleItem;
pub use components::merkle_path::MerklePath;
pub use iters::merkle_node_into_iter::MerkleNodeIntoIter;
pub use iters::merkle_node_iter::MerkleNodeIter;
pub use tree::merkle_node::MerkleNode;
pub use tree::merkle_tree::MerkleTree;
pub use tree::merkle_tree_builder::MerkleTreeBuilder;
#[cfg(feature = "encode")]
pub use utils::hex_encoding::bytes_to_hex;
#[cfg(feature = "encode")]
pub use utils::hex_encoding::Encodable;
pub use utils::algorithm;

mod components;
mod iters;
mod tree;
mod utils;
