/*!
Crate `merkle_hash` makes it easy to find the Blake3 hash
of files or entire directories using a Multithreaded Merkle tree algorithm


To use this crate, add `merkle_hash` as a dependency to your project's
`Cargo.toml`

```toml
[dependencies]
merkle_hash = "1"
```


# Example: Get the hash of a root directory using a merkle item
The following code creates a new merkle item with a given root directory path and then uses it to find the hash of that root
```
use std::path::PathBuf;
use merkle_hash::merkle_item::MerkleItem;

let path = PathBuf::from("/root/to/get/paths/from");
let merkle_item = MerkleItem::new(path);
let single_hash = merkle_item.get_hash();
```

# Example: Get the hash of a root directory using the merkle hashing functions
Using the utility functions instead of a merkle item to find the single hash of a root directory
```
use std::path::Path;
use merkle_hash::{get_paths,get_hashes,find_merkle_hash};

let root = Path::new("/root/to/get/paths/from");
let paths = get_paths(root);
let hashes = get_hashes(&paths);
let merkle_hash = find_merkle_hash(&hashes);
```


# Example: Get the merkle hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get the single merkle hash from a few blake3 hashes

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
 */

/// Simplifies the retrieval of hashes
pub mod merkle_item;
/// Utilities for using merkle hashing
pub mod merkle_utils;

pub use crate::merkle_item::MerkleItem;
pub use crate::merkle_utils::{find_merkle_hash, get_hashes, get_paths};

#[cfg(test)]
mod tests {
    use crate::merkle_item::MerkleItem;
    use crate::merkle_utils::{find_merkle_hash, get_hashes, get_paths};
    use blake3::{hash, Hash};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    #[test]
    fn test_find_merkle_hash() {
        let first_hash = hash(b"foo");
        let second_hash = hash(b"bar");
        let third_hash = hash(b"baz");
        let fourth_hash = hash(b"cow");
        let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

        let merkle_hash = find_merkle_hash(&hashes);

        let expected_hash_output =
            Hash::from_str("d804c32ea1170b63ba7ede0d1e32ca75640541a59360a183cc4462cedb571b36").ok();

        assert_eq!(merkle_hash, expected_hash_output);
    }

    #[test]
    fn test_hash_paths() {
        let root = Path::new("/root/to/get/paths/from");
        let paths = get_paths(root);
        let _hashes = get_hashes(&paths);
    }

    #[test]
    fn test_get_paths() {
        let root = Path::new("/root/to/get/paths/from");
        let _paths = get_paths(root);
    }

    #[test]
    fn test_merkle_item() {
        let path = PathBuf::from("/root/to/get/paths/from");
        let merkle_item = MerkleItem::new(path);
        let _single_hash = merkle_item.get_hash();
    }
}
