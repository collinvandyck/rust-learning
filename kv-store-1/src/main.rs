use std::{collections::HashMap, rc::Rc};

fn main() {
    let mut db = Db::default();
    db.set("key1", "val1");
    println!("{:?}", db.get("key1"));
    db.delete("key1");
    println!("{:?}", db.get("key1"));
}

#[derive(Default)]
struct Db {
    vals: HashMap<Key, Value>,
}

impl Db {
    fn get(&self, key: impl Into<Key>) -> Option<&Value> {
        let key = key.into();
        self.vals.get(&key)
    }

    fn set(&mut self, key: impl Into<Key>, val: impl Into<Value>) {
        let key = key.into();
        let val = val.into();
        self.vals.insert(key, val);
    }

    fn delete(&mut self, key: impl Into<Key>) {
        let key = key.into();
        self.vals.remove(&key);
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Key(Rc<str>);

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Key(Rc::from(value))
    }
}

#[derive(Debug)]
struct Value(Rc<str>);

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value(Rc::from(value))
    }
}
