use std::collections::{HashMap, HashSet};

fn main() {
    let mut ss = Spreadsheet::default();
    ss.set("A1", "32");
    ss.set("B1", "44");
    ss.set("A2", "A1");
    ss.display();
}

#[derive(Default)]
struct Spreadsheet {
    vals: HashMap<Key, Val>,
}

impl Spreadsheet {
    fn set<K, V>(&mut self, key: K, val: V)
    where
        K: Into<Key>,
        V: Into<Val>,
    {
        let key = key.into();
        let val = val.into();
        self.vals.insert(key, val);
    }

    fn display(&self) {
        let mut keys: Vec<&Key> = self.vals.keys().collect();
        keys.sort();
        for key in keys {
            let mut visited = HashSet::default();
            let val = self.evaluate(key, &mut visited);
            println!("{key}={val}");
        }
    }

    fn evaluate<'a>(&'a self, key: &'a Key, visited: &mut HashSet<&'a Key>) -> String {
        match self.vals.get(key) {
            Some(val) => match val {
                Val::Literal(val) => val.clone(),
                Val::Reference(val) => {
                    visited.insert(key);
                    self.evaluate(&val, visited)
                }
                Val::Formula(val) => val.clone(),
            },
            None => String::new(),
        }
    }
}

type Key = String;

enum Val {
    Literal(String),
    Reference(String),
    Formula(String),
}

impl From<String> for Val {
    fn from(value: String) -> Self {
        let chars: Vec<char> = value.trim().chars().collect();
        if !chars.is_empty() {
            if chars[0] >= 'A' && chars[0] <= 'Z' {
                return Val::Reference(value);
            }
            if chars[0] == '=' {
                return Val::Formula(chars[1..].iter().collect());
            }
        }
        Val::Literal(value)
    }
}

impl From<&str> for Val {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        Val::from(value)
    }
}
