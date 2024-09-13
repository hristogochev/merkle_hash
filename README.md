# merkle_hash

Finds the hashes of all files and directories in a directory tree.

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "3.7"
```

### Features

* Finds the master hash of a directory tree with ease.
* Offers multiple hashing algorithms.
* Allows including names in the hashing process.
* Uses a merkle tree algorithm to compute the hashes of directories.
* External iteration over the paths and hashes of files and directories.

### Limitations

* Currently only supports UTF-8 paths and will fail if a path is not UTF-8 encoded.

### Optional

* `sha` - Add this cargo feature to include `SHA-256` and `SHA-512` as hashing algorithms.
* `parallel` - Enabled by default, this feature makes the crate utilize all available threads.
* `encode` - Enabled by default, this feature adds the `bytes_to_hex` and `to_hex_string` functions.
* `retain` - Disabled by default, this feature duplicates the children paths of directories upon traversal.

### Examples

Get the master hash of a directory tree:

```rust,no_run
use merkle_hash::{algorithm::Blake3, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory")
    .algorithm(Algorithm::Blake3)
    .hash_names(false)
    .build()?;
let master_hash = tree.root.item.hash;
```

Iterate over a directory tree, getting the hash of each file and directory:

```rust,no_run
use merkle_hash::{Encodable, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
for item in tree {
    println!("{}: {}", item.path.relative, item.hash.to_hex_string());
}
```

Collapse the tree into any linear collection:

```rust,no_run
use std::collections::BTreeSet;
use merkle_hash::{MerkleItem, MerkleTree};

let tree = MerkleTree::builder("/path/to/directory").build()?;
let btree_set: BTreeSet<_> = tree.into_iter().collect();
```
### Release notes for 3.7
* Removed dependency on [anyhow](https://crates.io/crates/anyhow).
* Better error handling using a custom `IndexingError` type.

### Versioning

* Any major version of this crate may contain changes to the hashing algorithm.
* Any minor version of this crate may contain breaking changes to the API.
* Any patch version of this crate will only contain bug fixes and no breaking changes.

### Used technologies

* [rayon](https://crates.io/crates/rayon) for multithreaded directory reading and hashing.
* [camino](https://crates.io/crates/camino) to ensure that paths are always utf-8.
* [blake3](https://crates.io/crates/blake3) for the blake3 hashing of file contents.
* [sha2](https://crates.io/crates/sha2) for the sha256 and sha512 hashing of file contents.

### License

Licensed under [MIT license](https://github.com/hristogochev/merkle_hash/blob/main/LICENSE).