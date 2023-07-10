use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[allow(dead_code)]
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

#[allow(dead_code)]
impl<K, V> HashMap<K, V>
where
    K: Hash,
{
    pub fn new() -> Self {
        Self::new_size(8)
    }
    fn new_size(size: usize) -> Self {
        let mut buckets = vec![];
        for _ in 0..size {
            buckets.push(Bucket::new());
        }
        Self { buckets }
    }
    pub fn add(&mut self, key: K, val: V) {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        let hash = h.finish();
        let idx = hash % self.buckets.len() as u64;
        dbg!(idx);
    }
}

#[allow(dead_code)]
struct Bucket<K, V> {
    items: Vec<Item<K, V>>,
}

impl<K, V> Bucket<K, V> {
    fn new() -> Self {
        Self { items: vec![] }
    }
}

#[allow(dead_code)]
struct Item<K, V> {
    key: K,
    val: V,
}

#[allow(dead_code)]
impl<K, V> Item<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
}
