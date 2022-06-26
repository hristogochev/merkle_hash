# merkle_hash
Finds the blake3 hash of files or entire directories using a multithreaded merkle tree algorithm.

### Documentation

[docs.rs/merkle_hash](https://docs.rs/merkle_hash/)

### Usage

To use this crate, add `merkle_hash` as a dependency to your project's `Cargo.toml`:

```toml
[dependencies]
merkle_hash = "2"
```


### Example: Get the hashes of all files and directories descendants of a provided path as a traversable merkle tree

The following code demonstrates how to create a merkle tree of paths and hashes and then traverse it, executing a closure for each path and hash:

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let merkle_tree=MerkleTree::new("/provided/path").unwrap();
let traverse_result=merkle_tree.traverse(&|path,hash|{
    println!("{}: {}",path.absolute_path,hash);
    Ok(())
});
```

### Example: Get the hashes of all files and directories descendants of a provided path as a BTreeSet or as a HashSet

The following code demonstrates how to create a merkle tree of paths and hashes and then collapse it into a BTreeSet or a HashSet which can then be traversed linearly:

```rust,no_run
use merkle_hash::merkle_tree::MerkleTree;

let merkle_tree=MerkleTree::new("/provided/path").unwrap();
let convert_to_btree_set=true;
if convert_to_btree_set{
    let btree_set=merkle_tree.collapse_into_tree_set();
}else{
    let hash_set=merkle_tree.collapse_into_hash_set();
}
```

### Example: Get the hash of a collection of blake3 hashes

The following code demonstrates how to use the merkle hash function to get a single merkle hash from a few blake3 hashes:

```rust,no_run
use blake3::hash;
use merkle_hash::merkle_utils::compute_merkle_hash;

let first_hash = hash(b"foo");
let second_hash = hash(b"bar");
let third_hash = hash(b"baz");
let fourth_hash = hash(b"cow");
let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];

let merkle_hash = compute_merkle_hash(&hashes);
```