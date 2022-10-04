use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}