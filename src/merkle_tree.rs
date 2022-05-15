use crate::merkle_path::MerklePath;
use blake3::{Hash, Hasher};
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct MerkleTree {
    pub absolute_root: PathBuf,
    pub relative_root: PathBuf,
    pub cache: BTreeMap<MerklePath, Hash>,
}

impl MerkleTree {
    pub fn new(absolute_root: impl AsRef<Path>, relative_root: impl AsRef<Path>) -> Self {
        let absolute_root = absolute_root.as_ref().to_path_buf();
        let relative_root = relative_root.as_ref().to_path_buf();
        let cache = BTreeMap::new();
        Self {
            absolute_root,
            relative_root,
            cache,
        }
    }

    pub fn get_descendant_paths(&self, relative_path: impl AsRef<Path>) -> BTreeSet<MerklePath> {
        let root = self.absolute_root.join(&relative_path);

        let paths: BTreeSet<MerklePath> = WalkDir::new(&root)
            .into_iter()
            .flatten()
            .flat_map(|entry| {
                let absolute_path = entry.into_path();
                let relative_path = absolute_path
                    .strip_prefix(&self.absolute_root)
                    .ok()?
                    .to_path_buf();
                let node = MerklePath::new(relative_path, absolute_path);
                Some(node)
            })
            .collect();
        paths
    }

    pub fn get_hashes_from_disk<'a>(
        &self,
        paths: &'a BTreeSet<MerklePath>,
    ) -> BTreeMap<&'a MerklePath, Hash> {
        let hashes: BTreeMap<&MerklePath, Hash> = paths
            .into_par_iter()
            .flat_map(|path| self.get_hash_from_disk(path))
            .collect();
        hashes
    }

    pub fn get_hashes_from_cache<'a>(
        &self,
        paths: &'a BTreeSet<MerklePath>,
    ) -> BTreeMap<&'a MerklePath, Hash> {
        let output: BTreeMap<&MerklePath, Hash> = paths
            .into_par_iter()
            .flat_map(|path| self.get_hash_from_cache(path))
            .collect();
        output
    }

    pub fn get_hashes_combined<'a>(
        &self,
        paths: &'a BTreeSet<MerklePath>,
    ) -> BTreeMap<&'a MerklePath, Hash> {
        let output: BTreeMap<&MerklePath, Hash> = paths
            .into_par_iter()
            .flat_map(|path| match self.get_hash_from_cache(path) {
                None => self.get_hash_from_disk(path),
                Some(pair) => Some(pair),
            })
            .collect();
        output
    }

    pub fn get_hash_from_disk<'a>(&self, path: &'a MerklePath) -> Option<(&'a MerklePath, Hash)> {
        let path_str = path.relative_path.to_str()?;

        let mut hasher = Hasher::new();
        hasher.update(path_str.as_bytes());

        if path.absolute_path.is_file() {
            let file_bytes = fs::read(&path.absolute_path).ok()?;
            hasher.update(file_bytes.as_slice());
        }

        let hash = hasher.finalize();
        Some((path, hash))
    }

    pub fn get_hash_from_cache<'a>(&self, path: &'a MerklePath) -> Option<(&'a MerklePath, Hash)> {
        self.cache.get(path).map(|hash| (path, *hash))
    }

    pub fn cache_hashes(&mut self, hashes: &BTreeMap<&MerklePath, Hash>) {
        let hashes_cloned: BTreeMap<MerklePath, Hash> = hashes
            .into_par_iter()
            .map(|(path, hash)| ((*path).clone(), *hash))
            .collect();
        self.cache.extend(hashes_cloned)
    }
}
