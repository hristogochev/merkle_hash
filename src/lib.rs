/*!
Crate `merkle_hash` makes it easy to find the hashes of all files and directories in a directory tree.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3.4"
```

# Features

* Finds the master hash of a directory tree with ease.
* Offers multiple hashing algorithms.
* Allows including names in the hashing process.
* Uses a merkle tree algorithm to compute the hashes of directories.
* External iteration over the paths and hashes of files and directories.

# Optional

* `sha` - Add this cargo feature to include `SHA-256` and `SHA-512` as hashing algorithms.
* `parallel` - Enabled by default, this feature makes the crate utilize all available threads.


# Example: Get the master hash of a directory tree:
```
use merkle_hash::{MerkleTree,Algorithm};

let tree = MerkleTree::builder("/path/to/directory")
    .algorithm(Algorithm::Blake3)
    .hash_names(false)
    .build()?;
let master_hash = tree.root.item.hash;
```

# Example: Iterate over a directory tree, getting the hash of each file and directory:
```
use merkle_hash::{MerkleTree,bytes_to_hex};

let tree = MerkleTree::builder("/path/to/directory").build()?;
for item in tree {
    println!("{}: {}", item.path.relative, bytes_to_hex(&item.hash));
}
```

# Example: Collapse the tree into any linear collection:
```
use std::collections::BTreeSet;
use merkle_hash::{MerkleTree,MerkleItem};

let tree = MerkleTree::builder("/path/to/directory").build()?;
let btree_set: BTreeSet<MerkleItem> = tree.into_iter().collect();
```
 */

mod components;
mod iters;
mod tree;
mod utils;

pub use components::merkle_item::MerkleItem;
pub use components::merkle_path::MerklePath;
pub use iters::merkle_node_into_iter::MerkleNodeIntoIter;
pub use iters::merkle_node_iter::MerkleNodeIter;
pub use tree::merkle_node::MerkleNode;
pub use tree::merkle_tree::MerkleTree;
pub use tree::merkle_tree_builder::MerkleTreeBuilder;
pub use utils::algorithm::Algorithm;
pub use utils::hex_encoding::bytes_to_hex;

/// Used dependencies reexport
pub use anyhow;
pub use blake3;
pub use camino;
#[cfg(feature = "parallel")]
pub use rayon;
