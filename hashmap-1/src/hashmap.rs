#[allow(dead_code)]
pub struct HashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
}

#[allow(dead_code)]
impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self::new_size(8)
    }
    fn new_size(size: usize) -> Self {
        Self {
            buckets: Vec::with_capacity(size),
        }
    }
    pub fn add(&mut self, key: K, val: V) {}
}

#[allow(dead_code)]
struct Bucket<K, V> {
    items: Vec<Item<K, V>>,
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
