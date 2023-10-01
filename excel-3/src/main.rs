#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

fn main() {}

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
        if let Ok(v) = key.0.parse::<i64>() {
            return format!("{v}");
        }
        let val = self.vals.get(key);
        match val {
            Some(Value(s)) => {
                if visited.contains(s) {
                    return format!("ERROR");
                }
                if s.is_empty() {
                    return String::new();
                }
                if s.chars().nth(0).is_some_and(|t| t.is_alphabetic()) {
                    visited.insert(s.clone());
                    return self.eval(key, visited);
                }
                if s.chars().nth(0) == Some('=') {
                    let rest = &s[1..];
                    match rest.split_once('+') {
                        Some((left, right)) => {
                            visited.insert(s.clone());
                            let (left, right) = (Key(left.into()), Key(right.into()));
                            let left = self.eval(&left, visited);
                            let right = self.eval(&right, visited);
                            if let Ok(left) = left.parse::<i64>() {
                                if let Ok(right) = right.parse::<i64>() {
                                    let sum = left + right;
                                    return format!("{sum}");
                                }
                            }
                        }
                        None => {}
                    }
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
        assert_eq!(ss.set("a3", "=a1+a1"), "106");
        assert_eq!(ss.set("a4", "=a1+5"), "58");
        assert_eq!(ss.set("a5", "=a4+5"), "63");
    }
}
