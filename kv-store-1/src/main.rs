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
    parent: Option<Box<Db>>,
    vals: Option<HashMap<Key, Record>>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            parent: None,
            vals: Some(HashMap::default()),
        }
    }
}

impl Db {
    fn begin(&mut self) {}

    fn get(&self, key: impl Into<Key>) -> Option<&Value> {
        let key = key.into();
        let Some(vals) = &self.vals else { return None };
        vals.get(&key)
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
        let Some(vals) = self.vals.as_mut() else {
            return;
        };
        vals.insert(key, val);
    }

    fn delete(&mut self, key: impl Into<Key>) {
        let key = key.into();
        let Some(vals) = self.vals.as_mut() else {
            return;
        };
        if self.parent.is_some() {
            // we are in a tx.
            vals.insert(key, Record::Tombstone);
        } else {
            // we can just deleted.
            vals.remove(&key);
        }
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
