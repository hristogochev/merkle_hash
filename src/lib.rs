/*!
Crate `merkle_hash` makes it easy to find the blake3 hash
of files or entire directories using a multithreaded merkle tree algorithm.

# Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "2"
```

# Example: Get the hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get the single merkle hash from a few blake3 hashes:
```
use blake3::hash;
use merkle_hash::merkle_utils::compute_merkle_hash;

let first_hash = hash(b"foo");
let second_hash = hash(b"bar");
let third_hash = hash(b"baz");
let fourth_hash = hash(b"cow");
let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

let merkle_hash = compute_merkle_hash(&hashes);
```

# Example: Get the hashes of all files and directories descendants of a provided path as a traversable merkle tree
```
use merkle_hash::merkle_tree::MerkleTree;

let merkle_tree=MerkleTree::new("/provided/path").unwrap();
let traverse_result=merkle_tree.traverse(&|path,hash|{
    println!("{}: {}",path.absolute_path,hash);
    Ok(())
});
```

# Example: Get the hashes of all files and directories descendants of a provided path as a BTreeSet or as a HashSet
```
use merkle_hash::merkle_tree::MerkleTree;

let merkle_tree=MerkleTree::new("/provided/path").unwrap();
let convert_to_btree_set=true;
if convert_to_btree_set{
    let btree_set=merkle_tree.collapse_into_tree_set();
}else{
    let hash_set=merkle_tree.collapse_into_hash_set();
}
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
