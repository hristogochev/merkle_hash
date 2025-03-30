/*!
Crate `merkle_hash` makes it easy to find the hashes of all files and directories in a directory tree.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3.7"
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
* `camino` - Enabled by default, this feature makes all paths UTF-8 validated.
* `encode` - Enabled by default, this feature adds the `bytes_to_hex` and `to_hex_string` functions.
* `retain` - Disabled by default, this feature duplicates the children paths of directories upon traversal.
* `bincode` - Disabled by default, this feature enables bincode support.

# Example: Get the master hash of a directory tree:
```rust,no_run,ignore
use merkle_hash::{Algorithm, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory")
    .algorithm(Algorithm::Blake3)
    .hash_names(false)
    .build()?;
let master_hash = tree.root.item.hash;
```

# Example: Iterate over a directory tree, getting the hash of each file and directory:
```rust,no_run,ignore
use merkle_hash::{Encodable, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
for item in tree {
    println!("{}: {}", item.path.relative, item.hash.to_hex_string());
}
```

# Example: Collapse the tree into any linear collection:
```rust,no_run,ignore
use std::collections::BTreeSet;
use merkle_hash::{MerkleItem, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
let btree_set: BTreeSet<MerkleItem> = tree.into_iter().collect();
```
 */

/// Used dependencies reexport
pub use blake3;
#[cfg(feature = "camino")]
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
pub use utils::algorithm::Algorithm;
#[cfg(feature = "encode")]
pub use utils::hex_encoding::bytes_to_hex;
#[cfg(feature = "encode")]
pub use utils::hex_encoding::Encodable;

mod components;
mod iters;
mod tree;
mod utils;
/// Different types of errors for this crate
pub mod error;
