#![allow(dead_code)]
use std::{collections::HashMap, rc::Rc};

fn main() {}

#[test]
fn test_db() {
    let mut db = Db::default();
    db.set("key1", "val1");
    assert_eq!(db.get("key1"), Some(Value::from("val1")).as_ref());
    db.delete("key1");
    assert_eq!(db.get("key1"), None);
    assert_eq!(db.get("key_ne"), None);
}

#[test]
fn test_tx() {
    let mut db = Db::default();
    db.set("key0", "val0");
    assert_eq!(db.get("key0"), Some(Value::from("val0")).as_ref());
    db.begin();
    assert_eq!(db.get("key0"), Some(Value::from("val0")).as_ref());
    db.set("key1", "val1");
    assert_eq!(db.get("key1"), Some(Value::from("val1")).as_ref());
    db.commit();
    assert_eq!(db.get("key1"), Some(Value::from("val1")).as_ref());
}

#[test]
fn test_tx_2() {
    let mut db = Db::default();
    db.begin();
    db.set("key2", "val2");
    assert_eq!(db.get("key2"), Some(Value::from("val2")).as_ref());
    db.rollback();
    assert_eq!(db.get("key2"), None);
}

#[test]
fn test_tx_3() {
    let mut db = Db::default();
    db.set("key3", "val3");
    assert_eq!(db.get("key3"), Some(Value::from("val3")).as_ref());
    db.begin();
    db.delete("key3");
    assert_eq!(db.get("key3"), None);
    db.rollback();
    assert_eq!(db.get("key3"), Some(Value::from("val3")).as_ref());
}

#[test]
fn test_tx_4() {
    let mut db = Db::default();
    db.set("key3", "val3");
    assert_eq!(db.get("key3"), Some(Value::from("val3")).as_ref());
    db.begin();
    db.delete("key3");
    assert_eq!(db.get("key3"), None);
    db.commit();
    assert_eq!(db.get("key3"), None);
}

struct Db {
    stack: Vec<Storage>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            stack: vec![Storage::default()],
        }
    }
}

impl Db {
    fn begin(&mut self) {
        self.stack.push(Storage {
            tx: true,
            vals: Default::default(),
        });
    }

    fn commit(&mut self) {
        let storage = self.stack.pop().unwrap();
        let store = self.stack.last_mut().unwrap();
        store.merge(storage);
    }

    fn rollback(&mut self) {
        self.stack.pop();
    }

    fn get(&self, key: impl Into<Key>) -> Option<&Value> {
        let key = key.into();
        for stack in self.stack.iter().rev() {
            if let Some(record) = stack.get(&key) {
                return record.to_val();
            }
        }
        None
    }

    fn set(&mut self, key: impl Into<Key>, val: impl Into<Value>) {
        let storage = self.stack.last_mut().unwrap();
        storage.set(key, val);
    }

    fn delete(&mut self, key: impl Into<Key>) {
        let storage = self.stack.last_mut().unwrap();
        storage.delete(key);
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

impl Record {
    fn to_val(&self) -> Option<&Value> {
        match self {
            Record::Value(val) => Some(val),
            Record::Tombstone => None,
        }
    }
}

#[derive(Debug, PartialEq)]
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
    fn merge(&mut self, other: Storage) {
        for (k, v) in other.vals.into_iter() {
            match v {
                Record::Tombstone => {
                    self.vals.remove(&k);
                }
                _ => {
                    self.vals.insert(k, v);
                }
            };
        }
    }

    fn get(&self, key: &Key) -> Option<&Record> {
        self.vals.get(&key)
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
