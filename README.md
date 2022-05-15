# merkle_hash
Finds the blake3 hash of files or entire directories using a multithreaded merkle tree algorithm.

### Documentation

[docs.rs/merkle_hash](https://docs.rs/merkle_hash/)

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "1"
```


### Example: Get the hash of a root directory using a merkle tree

The following code uses a merkle tree to find the hash of a root directory:

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let mut merkle_tree=MerkleTree::new("/root/to/get/paths/from");
merkle_tree.index().unwrap();

let hash = merkle_tree.get_hash_for_path("").unwrap();
```

### Example: Get the hash of a subdirectory in a root using a merkle tree

The following code uses a merkle tree to find the hash of a subdirectory:

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let mut merkle_tree=MerkleTree::new("/root/to/get/paths/from");
merkle_tree.index().unwrap();

let hash = merkle_tree.get_hash_for_path("some/path/inside/the/root").unwrap();
```

### Example: Get the hash of a directory using a combination of the utility functions

The following code shows the simplest way to compute a merkle hash without having to instantiate a tree:

```rust,no_run
use merkle_hash::{get_paths,get_hashes_no_paths,find_merkle_hash};

let paths = get_paths("/root/to/get/paths/from").unwrap();
let hashes = get_hashes_no_paths(&paths).unwrap();

let merkle_hash = find_merkle_hash(&hashes);
```


### Example: Get the hash of a collection of blake3 hashes

The following code shows how to use the hashing function to get a single hash from a few blake3 hashes:

```rust,no_run
use blake3::hash;
use merkle_hash::merkle_utils::find_merkle_hash;

let first_hash = hash(b"foo");
let second_hash = hash(b"bar");
let third_hash = hash(b"baz");
let fourth_hash = hash(b"cow");
let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

let merkle_hash = find_merkle_hash(&hashes);
```