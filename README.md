# merkle_hash
Finds the hashes of all files and directories in a directory tree.

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3"
```

### Features

* Finds the master hash of a directory tree with ease.
* Uses a merkle tree algorithm to compute the hashes of directories.
* External iteration over the paths and hashes of each file and directory.
* Ability to specify whether names should be included in the hashes of files and directories.

### Examples

Get the master hash of a directory tree:

```rust,no_run
use merkle_hash::MerkleTree;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
let master_hash = tree.main_node.item.hash;
```

Iterate over a directory tree, getting the hash of each file and directory:

```rust,no_run
use merkle_hash::MerkleTree;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
for item in tree {
    println!("{}: {}", item.path.relative, item.hash);
}
```

Collapse the tree into any linear collection:

```rust,no_run
use std::collections::BTreeSet;
use merkle_hash::MerkleTree;
use merkle_hash::MerkleItem;

let tree = MerkleTree::new("/path/to/directory", true).unwrap();
let btree_set: BTreeSet<MerkleItem> = tree.into_iter().collect();
```


### Used technologies

* [rayon](https://crates.io/crates/rayon) for multithreaded directory reading and hashing.
* [blake3](https://crates.io/crates/blake3) for the hashing of file contents.
* [camino](https://crates.io/crates/camino) to ensure that paths are always utf-8.
* [anyhow](https://crates.io/crates/anyhow) to ease-out the handling of errors.

### License

Licensed under [MIT license](https://github.com/hristogochev/merkle_hash/blob/main/LICENSE).