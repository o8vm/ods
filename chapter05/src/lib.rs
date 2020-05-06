pub mod chainedhashtable;
pub mod linearhashtable;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hashcode<T: Hash>(x: &T) -> usize {
    let mut s = DefaultHasher::new();
    x.hash(&mut s);
    s.finish() as usize
}

pub fn byte_chunks_64(h: u64) -> [u8; 8] {
    [
        (h & 0xff) as u8,
        ((h >> 8) & 0xff) as u8,
        ((h >> 16) & 0xff) as u8,
        ((h >> 24) & 0xff) as u8,
        ((h >> 32) & 0xff) as u8,
        ((h >> 40) & 0xff) as u8,
        ((h >> 48) & 0xff) as u8,
        ((h >> 56) & 0xff) as u8,
    ]
}