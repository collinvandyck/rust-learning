use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq + Sized,
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
        let idx = self.index(&key);
        let bucket = self.buckets.get_mut(idx).unwrap();
        bucket.add(Item::new(key, val));
    }
    pub fn get(&self, key: K) -> Option<&V> {
        let idx = self.index(&key);
        let bucket = self.buckets.get(idx).unwrap();
        bucket.get(&key)
    }
    fn index(&self, key: &K) -> usize {
        let mut h = DefaultHasher::new();
        key.hash(&mut h);
        let hash = h.finish();
        hash as usize % self.buckets.len()
    }
}

struct Bucket<K, V> {
    items: Vec<Item<K, V>>,
}

impl<K, V> Bucket<K, V>
where
    K: Hash + PartialEq,
{
    fn new() -> Self {
        Self { items: vec![] }
    }
    fn add(&mut self, item: Item<K, V>) {
        for i in self.items.iter_mut() {
            if i.key == item.key {
                i.val = item.val;
                return;
            }
        }
        self.items.push(item);
    }
    fn get(&self, key: &K) -> Option<&V> {
        for i in self.items.iter() {
            if &i.key == key {
                return Some(&i.val);
            }
        }
        None
    }
}

struct Item<K, V> {
    key: K,
    val: V,
}

impl<K, V> Item<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
}
