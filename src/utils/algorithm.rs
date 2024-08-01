use blake3::Hasher;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "sha")]
use sha2::{Digest, Sha256 as Sha256Digest, Sha512 as Sha512Digest};

#[cfg(feature = "parallel")]
pub trait HashingAlgorithm<const N: usize>: Send + Sync {
    /// Computes a single hash from 2 slices of bytes
    fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> [u8; N];

    /// Computes a hash from a slice of bytes
    fn compute_hash(&self, bytes: &[u8]) -> [u8; N];
}

#[cfg(not(feature = "parallel"))]
pub trait HashingAlgorithm<const N: usize> {
    /// Computes a single hash from 2 slices of bytes
    fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> [u8; N];

    /// Computes a hash from a slice of bytes
    fn compute_hash(&self, bytes: &[u8]) -> [u8; N];
}

pub trait MerkleHashAlgorithm<const N: usize> {
    fn compute_merkle_hash(&self, hashes: &[&[u8; N]]) -> Option<[u8; N]>;
}

impl<const N: usize, T: HashingAlgorithm<N>> MerkleHashAlgorithm<N> for T
{
    fn compute_merkle_hash(&self, hashes: &[&[u8; N]]) -> Option<[u8; N]> {
        let len = hashes.len();

        if len < 1 {
            return None;
        }

        if len == 1 {
            return hashes.first().copied().copied();
        }

        #[cfg(feature = "parallel")]
            let chunks = hashes.par_chunks(2);

        #[cfg(not(feature = "parallel"))]
            let chunks = hashes.chunks(2);

        let output: Vec<[u8; N]> = chunks
            .flat_map(|hash_chunks: &[&[u8; N]]| {
                let first = hash_chunks.first()?;
                let second = hash_chunks.get(1).unwrap_or(first);
                let hash: [u8; N] = self.compute_hash_from_slices(first.as_slice(), second.as_slice());
                Some(hash)
            })
            .collect();


        let output: Vec<&[u8; N]> = output
            .iter()
            .collect();

        self.compute_merkle_hash(&output)
    }
}

pub struct Blake3;
impl HashingAlgorithm<32> for Blake3 {
    fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(first_slice);
        hasher.update(second_slice);
        hasher.finalize().into()
    }

    fn compute_hash(&self, bytes: &[u8]) -> [u8; 32] {
        blake3::hash(bytes).into()
    }
}

#[cfg(feature = "sha")]
pub struct Sha256;
#[cfg(feature = "sha")]
impl HashingAlgorithm<32> for Sha256 {
    fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256Digest::new();
        hasher.update(first_slice);
        hasher.update(second_slice);
        hasher.finalize().into()
    }

    fn compute_hash(&self, bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256Digest::new();

        hasher.update(bytes);

        hasher.finalize().into()
    }
}

#[cfg(feature = "sha")]
pub struct Sha512;
#[cfg(feature = "sha")]
impl HashingAlgorithm<64> for Sha512 {
    fn compute_hash_from_slices(&self, first_slice: &[u8], second_slice: &[u8]) -> [u8; 64] {
        let mut hasher = Sha512Digest::new();
        hasher.update(first_slice);
        hasher.update(second_slice);
        hasher.finalize().into()
    }

    fn compute_hash(&self, bytes: &[u8]) -> [u8; 64] {
        let mut hasher = Sha512Digest::new();

        hasher.update(bytes);

        hasher.finalize().into()
    }
}

// Hashing algorithms to choose from
// #[derive(Default)]
// pub enum Algorithm {
//     #[default]
//     Blake3,
//     #[cfg(feature = "sha")]
//     Sha256,
//     #[cfg(feature = "sha")]
//     Sha512,
// }
