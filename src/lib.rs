/*!
Crate `merkle_hash` makes it easy to find the blake3 hash
of files or entire directories using a multithreaded merkle tree algorithm.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "1"
```


# Example: Get the hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get the single merkle hash from a few blake3 hashes:
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

/// Utility item for managing merkle hashes
pub mod merkle_path;
pub mod merkle_tree;
/// Merkle hashing utility functions
pub mod merkle_utils;
pub use blake3;

#[cfg(test)]
mod tests {
    use crate::merkle_tree::MerkleTree;
    use std::path::PathBuf;

    #[test]
    fn attempt_caching() {
        let root =
            PathBuf::from("/run/media/scotty/Others/DataFromOldPC/DownloadsAndProjectsToSend");
        let mut tree = MerkleTree::new("/home/scotty/Desktop", "");
        let paths = tree.get_descendant_paths("");
        println!("{:?}", paths);
        let hashes = tree.get_hashes_from_disk(&paths);
        tree.cache_hashes(&hashes);
        let _hashes = tree.get_hashes_combined(&paths);
    }
}
