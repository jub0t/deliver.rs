use std::io::{self};

use xxhash_rust::const_xxh3::xxh3_64;
pub type HashSize = u64;

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hash(&self, buffer: Vec<u8>) -> io::Result<HashSize> {
        let hash = xxh3_64(&buffer);

        Ok(hash)
    }
}

pub struct Hasher {}
