use std::{borrow::Borrow, fmt::Debug, hash::Hash};

fn main() {
    let m: Map<String, String> = Map::new();
    dbg!(m.get("foo"));
    let s = String::from("foobar");
    dbg!(m.get(&s));
}

struct Map<K, V> {
    _k: Option<K>,
    _v: Option<V>,
}

impl<K, V> Map<K, V> {
    pub fn new() -> Self {
        Self { _k: None, _v: None }
    }
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Debug + Hash + Eq + ?Sized,
    {
        dbg!(k.borrow());
        print_type_of(&k);
        // ...
        None
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
