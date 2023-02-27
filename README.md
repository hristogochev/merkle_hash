# merkle_hash
Finds the hashes of all files and directories in a directory tree.

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3.2"
```

### Features

* Finds the master hash of a directory tree with ease.
* Offers [Blake3](https://crates.io/crates/blake3) and [SHA-256](https://crates.io/crates/sha2) as hashing algorithms.
* Offers ability to include names in the hashing process.
* Uses a merkle tree algorithm to compute the hashes of directories.
* External iteration over the paths and hashes of files and directories.


### Examples

Get the master hash of a directory tree:

```rust,no_run
use merkle_hash::{MerkleTree,Algorithm};

let tree = MerkleTree::builder("/path/to/directory")
    .algorithm(Algorithm::Blake3)
    .hash_names(false)
    .build()?;
let master_hash = tree.main_node.item.hash;
```

Iterate over a directory tree, getting the hash of each file and directory:

```rust,no_run
use merkle_hash::{MerkleTree,bytes_to_hex};

let tree = MerkleTree::builder("/path/to/directory").build()?;
for item in tree {
    println!("{}: {}", item.path.relative, bytes_to_hex(&item.hash));
}
```

Collapse the tree into any linear collection:

```rust,no_run
use std::collections::BTreeSet;
use merkle_hash::{MerkleTree,MerkleItem};

let tree = MerkleTree::builder("/path/to/directory").build()?;
let btree_set: BTreeSet<MerkleItem> = tree.into_iter().collect();
```


### Used technologies

* [rayon](https://crates.io/crates/rayon) for multithreaded directory reading and hashing.
* [camino](https://crates.io/crates/camino) to ensure that paths are always utf-8.
* [anyhow](https://crates.io/crates/anyhow) to ease-out the handling of errors.
* [blake3](https://crates.io/crates/blake3) for the blake3 hashing of file contents.
* [sha2](https://crates.io/crates/sha2) for the sha256 hashing of file contents.

### License

Licensed under [MIT license](https://github.com/hristogochev/merkle_hash/blob/main/LICENSE).