use crate::merkle_node::MerkleNode;
use anyhow::{Context, Result};
use blake3::{Hash, Hasher};
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Same as find_merkle_hash with the exception that it takes a vec of references to hashes
pub fn find_merkle_hash_from_refs(hashes: Vec<&Hash>) -> Option<Hash> {
    let len = hashes.len();

    if len < 1 {
        return None;
    }

    if len == 1 {
        return hashes.first().cloned().cloned();
    }

    let output: Vec<Hash> = hashes
        .par_chunks(2)
        .flat_map(|hash_chunks| {
            let first = hash_chunks.get(0)?.as_bytes();
            let second = match hash_chunks.get(1).map(|hash| hash.as_bytes()) {
                Some(second) => second,
                None => first,
            };
            let mut hasher = Hasher::new();
            hasher.update(first);
            hasher.update(second);
            let hash = hasher.finalize();
            Some(hash)
        })
        .collect();

    find_merkle_hash(&output)
}

/// Finds the hash of a slice of hashes using a merkle tree algorithm in multithreaded mode
///
/// - If the provided slice is empty, returns None
/// - If the provided slice is not empty, recursively finds the single hash
pub fn find_merkle_hash(hashes: &[Hash]) -> Option<Hash> {
    let len = hashes.len();

    if len < 1 {
        return None;
    }

    if len == 1 {
        return hashes.first().cloned();
    }

    let output: Vec<Hash> = hashes
        .par_chunks(2)
        .flat_map(|hash_chunks| {
            let first = hash_chunks.get(0)?.as_bytes();
            let second = match hash_chunks.get(1).map(Hash::as_bytes) {
                Some(second) => second,
                None => first,
            };
            let mut hasher = Hasher::new();
            hasher.update(first);
            hasher.update(second);
            let hash = hasher.finalize();
            Some(hash)
        })
        .collect();

    find_merkle_hash(&output)
}

/// Hashes all given files and directories from BTreeMap collection of paths
/// without associating them with relative paths
///
/// - If a path is to a directory, hashes the directory by name
/// - If a path is to a file, hashes the file by name and contents
pub fn get_hashes_no_paths(paths: &BTreeMap<PathBuf, PathBuf>) -> Result<Vec<Hash>> {
    paths
        .par_iter()
        .map(|(relative_path, absolute_path)| {
            let path_str = relative_path.to_str().with_context(|| {
                format!("Could not compute hash for: {}", relative_path.display())
            })?;

            let mut hasher = Hasher::new();
            hasher.update(path_str.as_bytes());

            if absolute_path.is_file() {
                let file_bytes = fs::read(absolute_path)?;
                hasher.update(file_bytes.as_slice());
            }

            let hash = hasher.finalize();
            Ok(hash)
        })
        .collect()
}

/// Hashes all given files and directories from BTreeMap collection of paths
///
/// - If a path is to a directory, hashes the directory by name
/// - If a path is to a file, hashes the file by name and contents
pub fn get_hashes(paths: BTreeMap<PathBuf, PathBuf>) -> Result<BTreeSet<MerkleNode>> {
    paths
        .into_par_iter()
        .map(|(relative_path, absolute_path)| {
            let path_str = relative_path.to_str().with_context(|| "Hashing error")?;

            let mut hasher = Hasher::new();
            hasher.update(path_str.as_bytes());

            if absolute_path.is_file() {
                let file_bytes = fs::read(&absolute_path)?;
                hasher.update(file_bytes.as_slice());
            }

            let hash = hasher.finalize();

            Ok(MerkleNode::new(relative_path, absolute_path, hash))
        })
        .collect()
}

/// Retrieves a BTreeMap collection of all paths descendants of a given root
///
/// Each pair in the BTreeMap contains
/// a path relative to the root as a key
/// and an absolute path as a value
pub fn get_paths(root: impl AsRef<Path>) -> Result<BTreeMap<PathBuf, PathBuf>> {
    WalkDir::new(&root)
        .into_iter()
        .map(|entry| {
            let absolute_path = entry?.into_path();
            let relative_path = absolute_path.strip_prefix(&root)?.to_path_buf();
            Ok((relative_path, absolute_path))
        })
        .collect()
}
