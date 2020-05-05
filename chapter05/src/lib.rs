pub mod chainedhashtable;
pub mod linearhashtable;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hashcode<T: Hash>(x: &T) -> usize {
    let mut s = DefaultHasher::new();
    x.hash(&mut s);
    s.finish() as usize
}
