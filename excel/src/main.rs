use std::collections::{HashMap, HashSet};

fn main() {
    let mut ss = Spreadsheet::default();
    ss.set("A1", "32");
    ss.set("B1", "44");
    ss.set("A2", "A1");
    ss.set("N1", "=A1+B1");
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

    fn evaluate(&self, key: &Key, visited: &mut HashSet<Key>) -> String {
        if visited.contains(key) {
            return String::new();
        }
        visited.insert(key.to_string());
        self.vals
            .get(key)
            .map(|val| match val {
                Val::Literal(val) => val.clone(),
                Val::Reference(val) => self.evaluate(&val, visited),
                Val::Formula(val) => {
                    let parts: Vec<&str> = val.split('+').collect();
                    match parts[..] {
                        [first, second] => {
                            let first = self.evaluate(&first.to_string(), visited);
                            let second = self.evaluate(&second.to_string(), visited);
                            Self::add(first, second).unwrap_or(val.clone())
                        }
                        _ => val.clone(),
                    }
                }
            })
            .unwrap_or_default()
    }

    fn add(o1: String, o2: String) -> Option<String> {
        if let Ok(o1) = o1.parse::<i64>() {
            if let Ok(o2) = o2.parse::<i64>() {
                let sum = o1 + o2;
                return Some(format!("{sum}"));
            }
        }
        None
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
