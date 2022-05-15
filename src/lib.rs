/*!
Crate `merkle_hash` makes it easy to find the blake3 hash
of files or entire directories using a multithreaded merkle tree algorithm.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "1"
```


# Example: Get the hash of a root directory using a merkle tree
The following code uses a merkle tree to find the hash of a root directory:
```
use merkle_hash::merkle_tree::MerkleTree;

let mut merkle_tree=MerkleTree::new("/root/to/get/paths/from");

let hash = merkle_tree.get_hash_for_path("").unwrap();
```

# Example: Get the hash of a subdirectory in a root using a merkle tree
The following code uses a merkle tree to find the hash of a subdirectory:
```
use merkle_hash::merkle_tree::MerkleTree;

let mut merkle_tree=MerkleTree::new("/root/to/get/paths/from");

let hash = merkle_tree.get_hash_for_path("some/path/inside/the/root").unwrap();
```

# Example: Get the hash of a directory using a combination of the utility functions
The following code shows the simplest way to compute a merkle hash without having to instantiate a tree:
```
use merkle_hash::{get_paths,get_hashes_no_paths,find_merkle_hash};

let paths = get_paths("/root/to/get/paths/from").unwrap();
let hashes = get_hashes_no_paths(&paths).unwrap();

let merkle_hash = find_merkle_hash(&hashes);
```


# Example: Get the hash of a collection of blake3 hashes
The following code shows how to use the hashing function to get a single hash from a few blake3 hashes:
```
use blake3::hash;
use merkle_hash::merkle_utils::find_merkle_hash;

let first_hash = hash(b"foo");
let second_hash = hash(b"bar");
let third_hash = hash(b"baz");
let fourth_hash = hash(b"cow");
let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

let merkle_hash = find_merkle_hash(&hashes);
```


 */

mod merkle_node;
/// Merkle tree implementation
pub mod merkle_tree;
/// Merkle hashing utility functions
pub mod merkle_utils;

pub use crate::merkle_tree::MerkleTree;
pub use crate::merkle_utils::{find_merkle_hash, get_hashes, get_hashes_no_paths, get_paths};
pub use blake3;
pub use rayon;
