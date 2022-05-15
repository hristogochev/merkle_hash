use crate::merkle_node::MerkleNode;
use anyhow::{anyhow, Context, Result};
use blake3::{Hash, Hasher};
use parking_lot::Mutex;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::fs::read_dir;
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

#[derive(Eq, PartialEq, Debug)]
pub struct NewMerkleNode {
    pub absolute_path: PathBuf,
    pub relative_path: PathBuf,
    pub children: BTreeSet<NewMerkleNode>,
    pub hash: Hash,
}

impl NewMerkleNode {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let root = path.as_ref();
        let path = root.to_path_buf();
        Self::get_node(root, path)
    }
    fn get_node(root: &Path, absolute_path: PathBuf) -> Result<NewMerkleNode> {
        let children: BTreeSet<NewMerkleNode> = if absolute_path.is_dir() {
            let children: Result<BTreeSet<NewMerkleNode>> = read_dir(&absolute_path)?
                .par_bridge()
                .map(|entry| {
                    let child_absolute_path = entry?.path();
                    let child = Self::get_node(root, child_absolute_path)?;
                    Ok(child)
                })
                .collect();
            children?
        } else {
            BTreeSet::new()
        };

        let relative_path = absolute_path.strip_prefix(root)?.to_path_buf();

        let relative_path_str = relative_path
            .to_str()
            .with_context(|| format!("Could not compute hash for: {}", relative_path.display()))?;

        let mut hasher = Hasher::new();
        hasher.update(relative_path_str.as_bytes());

        if absolute_path.is_dir() {
            let child_hashes: Vec<Hash> = children.iter().map(|child| child.hash).collect();
            if let Some(children_hash) = find_merkle_hash(&child_hashes) {
                hasher.update(children_hash.as_bytes());
            }
        } else {
            let file_bytes = fs::read(&absolute_path)?;
            hasher.update(file_bytes.as_slice());
        }

        let hash = hasher.finalize();
        Ok(NewMerkleNode {
            absolute_path,
            relative_path,
            children,
            hash,
        })
    }

    pub fn get_name(&self) -> Option<&str> {
        self.absolute_path.file_name()?.to_str()
    }

    // pub fn get_hash(&self) -> Result<Hash> {
    //     let mut hasher = Hasher::new();
    //     let path_str = self.relative_path.to_str().with_context(|| {
    //         format!(
    //             "Could not compute hash for: {}",
    //             self.relative_path.display()
    //         )
    //     })?;
    //     hasher.update(path_str.as_bytes());
    //
    //     if self.absolute_path.is_file() {
    //         let file_bytes = fs::read(&self.absolute_path)?;
    //         hasher.update(file_bytes.as_slice());
    //     } else {
    //         let child_hashes: Result<Vec<Hash>> =
    //             self.children.iter().map(|child| child.get_hash()).collect();
    //         if let Some(children_hash) = find_merkle_hash(&child_hashes?) {
    //             hasher.update(children_hash.as_bytes());
    //         }
    //     }
    //
    //     Ok(hasher.finalize())
    // }
}

impl PartialOrd<Self> for NewMerkleNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.absolute_path.partial_cmp(&other.absolute_path)
    }
}

impl Ord for NewMerkleNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.absolute_path.cmp(&other.absolute_path)
    }
}
