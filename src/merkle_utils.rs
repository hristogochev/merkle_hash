use blake3::{Hash, Hasher};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Finds the merkle hash of a file or a directory
///
/// - If the provided path is valid, recursively finds the single hash
/// - If the provided path is invalid, returns None
///
/// # Examples
///
/// ```
/// use merkle_hash::merkle_utils::get_merkle_hash;
///
/// let merkle_hash = get_merkle_hash("/root/to/get/paths/from");
/// ```
pub fn get_merkle_hash(path: impl AsRef<Path>) -> Option<Hash> {
    let tree = get_paths(path);
    let hashes = get_hashes(&tree);
    find_merkle_hash(&hashes)
}

/// Finds the hash of a slice of hashes using a merkle tree algorithm in multithreaded mode
///
/// - If the provided slice is empty, returns None
/// - If the provided slice is not empty, recursively finds the single hash
///
/// # Examples
///
/// ```
///
/// use std::str::FromStr;
/// use blake3::{hash,Hash};
/// use merkle_hash::merkle_utils::find_merkle_hash;
///
/// let first_hash = hash(b"foo");
/// let second_hash = hash(b"bar");
/// let third_hash = hash(b"baz");
/// let fourth_hash = hash(b"cow");
/// let hashes = vec![first_hash, second_hash, third_hash, fourth_hash];
///
/// let merkle_hash = find_merkle_hash(&hashes);
///
/// let expected_hash_output =
/// Hash::from_str("d804c32ea1170b63ba7ede0d1e32ca75640541a59360a183cc4462cedb571b36").ok();
///
/// assert_eq!(merkle_hash, expected_hash_output);
/// ```
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

/// Hashes all given files and directories from BTreeMap collection of paths in in multithreaded mode using blake3 hashing
///
/// - If a path is to a directory, hashes the directory by name
/// - If a path is to a file, hashes the file by name and contents
///
/// # Examples
///
/// ```
/// use merkle_hash::merkle_utils::{get_paths, get_hashes};
///
/// let paths = get_paths("/root/to/get/paths/from");
/// let hashes = get_hashes(&paths);
/// ```
pub fn get_hashes(paths: &BTreeMap<PathBuf, PathBuf>) -> Vec<Hash> {
    let hashes: Vec<Hash> = paths
        .par_iter()
        .flat_map(|(relative_path, absolute_path)| {
            let path_str = relative_path.to_str()?;

            let mut hasher = blake3::Hasher::new();
            hasher.update(path_str.as_bytes());

            if absolute_path.is_file() {
                let file_bytes = fs::read(absolute_path).ok()?;
                hasher.update(file_bytes.as_slice());
            }

            let hash = hasher.finalize();
            Some(hash)
        })
        .collect();
    hashes
}

/// Retrieves a BTreeMap collection of all paths inside a given root
///
/// Each pair in the BTreeMap contains
/// a path relative to the root as a key
/// and an absolute path as a value
///
/// # Examples
///
/// ```
/// use merkle_hash::merkle_utils::get_paths;
///
/// let paths = get_paths("/root/to/get/paths/from");
/// ```
pub fn get_paths(root: impl AsRef<Path>) -> BTreeMap<PathBuf, PathBuf> {
    let paths: BTreeMap<PathBuf, PathBuf> = WalkDir::new(&root)
        .into_iter()
        .flatten()
        .flat_map(|entry| {
            let absolute_path = entry.into_path();
            let relative_path = absolute_path.strip_prefix(&root).ok()?.to_path_buf();
            Some((relative_path, absolute_path))
        })
        .collect();
    paths
}
