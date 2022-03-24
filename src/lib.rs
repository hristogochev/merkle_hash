/*!
Crate `merkle_hash` makes it easy to find the blake3 hash
of files or entire directories using a multithreaded merkle tree algorithm.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "1"
```

# Example: Get the hash of a directory using a single function
The following code shows the simplest way to get the merkle hash of a directory:
```
use std::path::Path;
use merkle_hash::merkle_utils::get_merkle_hash;

let path = Path::new("/root/to/get/paths/from");
let merkle_hash = get_merkle_hash(path);
```

# Example: Get the hash of a directory using a combination of the utility functions
The following code shows the more complicated way to get the merkle hash:
```
use std::path::Path;
use merkle_hash::{get_paths,get_hashes,find_merkle_hash};

let root = Path::new("/root/to/get/paths/from");
let paths = get_paths(root);
let hashes = get_hashes(&paths);
let merkle_hash = find_merkle_hash(&hashes);
```


# Example: Get the hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get the single merkle hash from a few blake3 hashes:
```
use std::str::FromStr;
use blake3::{hash,Hash};
use merkle_hash::merkle_utils::find_merkle_hash;

let first_hash = hash(b"foo");
let second_hash = hash(b"bar");
let third_hash = hash(b"baz");
let fourth_hash = hash(b"cow");
let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

let merkle_hash = find_merkle_hash(&hashes);
```


# Example: Get the hash of a directory using a merkle item
The following code uses a merkle item to find the merkle hash of a directory,
you may want to use a merkle item if you want access to more of its functions such as getting all of its direct descendants and more:
```
use std::path::PathBuf;
use merkle_hash::merkle_item::MerkleItem;

let path = PathBuf::from("/root/to/get/paths/from");
let merkle_item = MerkleItem::new(path);
let merkle_hash = merkle_item.get_hash();
```
 */

/// Utility item for managing merkle hashes
pub mod merkle_item;
/// Merkle hashing utility functions
pub mod merkle_utils;

pub use crate::merkle_item::MerkleItem;
pub use crate::merkle_utils::{find_merkle_hash, get_hashes, get_merkle_hash, get_paths};
