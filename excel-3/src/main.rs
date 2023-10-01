#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Default)]
struct Spreadsheet {
    vals: HashMap<Key, Value>,
}

impl Spreadsheet {
    fn set(&mut self, key: impl Into<Rc<str>>, val: impl Into<Rc<str>>) {
        let key = Key(key.into());
        let val = Value(val.into());
        self.vals.insert(key, val);
    }

    fn get(&self, key: impl Into<Rc<str>>) -> String {
        let key = Key(key.into());
        let mut visited = HashSet::default();
        self.eval(&key, &mut visited)
    }

    fn eval(&self, key: &Key, visited: &mut HashSet<Rc<str>>) -> String {
        let val = self.vals.get(key);
        match val {
            Some(val) => val.to_string(),
            None => String::new(),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Key(Rc<str>);

impl Deref for Key {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Value(Rc<str>);

impl Deref for Value {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ss() {
        let mut ss = Spreadsheet::default();
        ss.set("a1", "53");
        assert_eq!(ss.get("a1"), "53");
    }
}
