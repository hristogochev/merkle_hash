# merkle_hash
Finds the hashes of all files and directories in a directory tree.

### Documentation

[docs.rs/merkle_hash](https://docs.rs/merkle_hash/)

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "2"
```


#### Example: Get the master hash of a directory tree

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let tree = MerkleTree::new("path/to/tree").unwrap();
let master_hash = tree.main_node.hash;
```

#### Example: Traverse a directory tree, getting the hash of each file and directory

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let tree = MerkleTree::new("/path/to/tree").unwrap();
let traverse_result = tree.traverse(&|path,hash|{
    println!("{}: {}", path.absolute_path, hash);
    Ok(())
});
```

#### Example: Collapse the tree for linear traversal

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let tree = MerkleTree::new("/path/to/tree").unwrap();
let btree_set = tree.collapse_into_tree_set();
```