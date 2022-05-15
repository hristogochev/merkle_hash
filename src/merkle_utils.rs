use blake3::{Hash, Hasher};
use rayon::prelude::*;

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
