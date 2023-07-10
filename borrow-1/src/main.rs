use std::{borrow::Borrow, fmt::Debug};

fn main() {
    let mut list: List<String, String> = List::new();
    list.add("foo", "bar");
    let k = "bar";
    let v = "baz";
    list.add(k, v);
    list.add(k, v);
    let k = "abc".to_string();
    let v = "foobar";
    list.add(&k, v);
    dbg!(k);
    list = dbg!(list);
    dbg!(list.get("bar"));
}

#[derive(Debug)]
struct List<K, V> {
    items: Vec<Item<K, V>>,
}

#[derive(Debug)]
struct Item<K, V>(K, V);

impl<K, V> Item<K, V> {
    fn new(k: K, v: V) -> Self {
        Self(k, v)
    }
    fn key(&self) -> &K {
        &self.0
    }
    fn val(&self) -> &V {
        &self.1
    }
}

impl<K, V> List<K, V> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    fn add<KR, KV>(&mut self, k: KR, v: KV)
    where
        KR: Into<K>,
        KV: Into<V>,
    {
        self.items.push(Item::new(k.into(), v.into()));
    }

    fn get<Q>(&self, k: &Q) -> Vec<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq + Eq + ?Sized,
    {
        self.items
            .iter()
            .filter(|i| i.key().borrow() == k)
            .map(|i| i.val())
            .collect::<Vec<_>>()
    }
}

fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
