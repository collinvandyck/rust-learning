use std::{collections::HashMap, rc::Rc};

fn main() {
    let mut db = Db::default();
    db.set("key1", "val1");
    println!("{:?}", db.get("key1"));
    db.delete("key1");
    println!("{:?}", db.get("key1"));
    println!("{:?}", db.get("key_ne"));
}

struct Db {
    parent: Option<Storage>,
    vals: Option<Storage>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            parent: None,
            vals: Some(Storage::default()),
        }
    }
}

impl Db {
    fn begin(&mut self) {}

    fn get(&self, key: impl Into<Key>) -> Option<&Value> {
        let Some(vals) = &self.vals else { return None };
        vals.get(key)
    }

    fn set(&mut self, key: impl Into<Key>, val: impl Into<Value>) {
        let Some(vals) = self.vals.as_mut() else {
            return;
        };
        vals.set(key, val);
    }

    fn delete(&mut self, key: impl Into<Key>) {
        let Some(vals) = self.vals.as_mut() else {
            return;
        };
        vals.delete(key);
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Key(Rc<str>);

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Key(Rc::from(value))
    }
}

enum Record {
    Value(Value),
    Tombstone,
}

#[derive(Debug)]
struct Value(Rc<str>);

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value(Rc::from(value))
    }
}

#[derive(Default)]
struct Storage {
    tx: bool,
    vals: HashMap<Key, Record>,
}

impl Storage {
    fn get(&self, key: impl Into<Key>) -> Option<&Value> {
        let key = key.into();
        self.vals
            .get(&key)
            .map(|r| match r {
                Record::Value(val) => Some(val),
                Record::Tombstone => None,
            })
            .flatten()
    }

    fn set(&mut self, key: impl Into<Key>, val: impl Into<Value>) {
        let key = key.into();
        let val = val.into();
        let val = Record::Value(val);
        self.vals.insert(key, val);
    }

    fn delete(&mut self, key: impl Into<Key>) {
        let key = key.into();
        if self.tx {
            self.vals.insert(key, Record::Tombstone);
        } else {
            self.vals.remove(&key);
        }
    }
}
