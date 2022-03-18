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

### Example: Get the hash of a root directory using a merkle item

The following code creates a new merkle item with a given root directory path and then uses it to find the hash of that root:

```rust,no_run
use std::path::PathBuf;
use merkle_hash::merkle_item::MerkleItem;

let path = PathBuf::from("/root/to/get/paths/from");
let merkle_item = MerkleItem::new(path);
let single_hash = merkle_item.get_hash();
```

### Example: Get the hash of a root directory using the merkle hashing functions

Using the utility functions instead of a merkle item to find the single hash of a root directory:

```rust,no_run
use std::path::Path;
use merkle_hash::{get_paths,get_hashes,find_merkle_hash};

let root = Path::new("/root/to/get/paths/from");
let paths = get_paths(root);
let hashes = get_hashes(&paths);
let merkle_hash = find_merkle_hash(&hashes);
```


### Example: Get the merkle hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get the single merkle hash from a few blake3 hashes:

```rust,no_run
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
