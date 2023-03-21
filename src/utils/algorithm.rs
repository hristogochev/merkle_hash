use blake3::Hasher;
use rayon::prelude::*;
#[cfg(feature = "sha")]
use sha2::{Digest, Sha256, Sha512};

/// Hashing algorithms to choose from
#[derive(Default)]
pub enum Algorithm {
    #[default]
    Blake3,
    #[cfg(feature = "sha")]
    Sha256,
    #[cfg(feature = "sha")]
    Sha512,
}

impl Algorithm {
    /// Computes a merkle hash from a slice of bytes
    pub fn compute_merkle_hash(&self, hashes: &[&[u8]]) -> Option<Vec<u8>> {
        let len = hashes.len();

        if len < 1 {
            return None;
        }

        if len == 1 {
            return hashes.first().copied().map(|first| first.to_vec());
        }

        let output: Vec<_> = hashes
            .par_chunks(2)
            .flat_map(|hash_chunks| {
                let first = hash_chunks.first()?;
                let second = match hash_chunks.get(1) {
                    Some(second) => second,
                    None => first,
                };
                let hash = self.compute_hash_from_slices(first, second);
                Some(hash)
            })
            .collect();

        let output: Vec<_> = output
            .iter()
            .map(|reference| reference.as_slice())
            .collect();

        self.compute_merkle_hash(&output)
    }

    /// Computes a single hash from 2 slices of bytes
    pub fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> Vec<u8> {
        match self {
            Algorithm::Blake3 => {
                let mut hasher = Hasher::new();
                hasher.update(first_slice);
                hasher.update(second_slice);
                hasher.finalize().as_bytes().to_vec()
            }
            #[cfg(feature = "sha")]
            Algorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(first_slice);
                hasher.update(second_slice);
                hasher.finalize().to_vec()
            }
            #[cfg(feature = "sha")]
            Algorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(first_slice);
                hasher.update(second_slice);
                hasher.finalize().to_vec()
            }
        }
    }

    /// Computes a hash from a slice of bytes
    pub fn compute_hash(&self, bytes: &[u8]) -> Vec<u8> {
        match self {
            Algorithm::Blake3 => blake3::hash(bytes).as_bytes().to_vec(),
            #[cfg(feature = "sha")]
            Algorithm::Sha256 => {
                let mut hasher = Sha256::new();

                hasher.update(bytes);

                hasher.finalize().to_vec()
            }
            #[cfg(feature = "sha")]
            Algorithm::Sha512 => {
                let mut hasher = Sha512::new();

                hasher.update(bytes);

                hasher.finalize().to_vec()
            }
        }
    }
}
