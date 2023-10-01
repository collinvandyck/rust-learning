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
    fn set(&mut self, key: impl Into<Rc<str>>, val: impl Into<Rc<str>>) -> String {
        let key: Rc<str> = key.into();
        let key = Key(key.clone());
        let val = Value(val.into());
        self.vals.insert(key.clone(), val);
        self.get(key.0)
    }

    fn get(&self, key: impl Into<Rc<str>>) -> String {
        let key = Key(key.into());
        let mut visited = HashSet::default();
        self.eval(&key, &mut visited)
    }

    fn eval(&self, key: &Key, visited: &mut HashSet<Rc<str>>) -> String {
        let val = self.vals.get(key);
        match val {
            Some(Value(s)) => {
                if let Ok(v) = s.parse::<i64>() {
                    return format!("{v}");
                }
                if visited.contains(s) {
                    return format!("ERROR");
                }
                let ident = s.chars().take(1).all(|f| f.is_alphabetic());
                if ident {
                    visited.insert(s.clone());
                    return self.eval(key, visited);
                }
                s.to_string()
            }
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
        assert_eq!(ss.set("a1", "53"), "53");
        assert_eq!(ss.set("a2", "a2"), "ERROR");
        assert_eq!(ss.get("a2"), "ERROR");
    }
}
