#![allow(dead_code, unused)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn main() {}

#[derive(Default)]
struct Spreadsheet {
    vals: HashMap<Address, Value>,
}

impl Spreadsheet {
    fn set<K, V>(&mut self, key: K, val: V) -> String
    where
        K: Into<Address>,
        V: Into<Value>,
    {
        let key: Address = key.into();
        self.vals.insert(key.clone(), val.into());
        self.get(key)
    }

    fn get<K>(&self, key: K) -> String
    where
        K: Into<Address>,
    {
        let mut visited = HashSet::default();
        self.fetch(key, &mut visited)
    }

    fn fetch<A>(&self, key: A, visited: &mut HashSet<Address>) -> String
    where
        A: Into<Address>,
    {
        let key = key.into();
        if let Some(v) = Self::as_f64(&key.0) {
            return format!("{v}");
        }
        let Some(val) = self.vals.get(&key) else {
            return String::new();
        };
        if visited.contains(&key) {
            return String::new();
        }
        match val {
            Value::String(s) => return s.clone(),
            Value::Address(addr) => {
                visited.insert(key.clone());
                return self.fetch(addr.clone(), visited);
            }
            Value::Number(num) => format!("{num}"),
            Value::Formula(formula) => {
                match formula.chars().skip(1).collect::<String>().split_once('+') {
                    Some((left, right)) => {
                        visited.insert(key.clone());
                        let left = self.fetch(left, visited);
                        let right = self.fetch(right, visited);
                        if let Some(left) = Self::as_f64(&left) {
                            if let Some(right) = Self::as_f64(&right) {
                                let sum = left + right;
                                return format!("{sum}");
                            }
                        }
                        return String::new();
                    }
                    _ => String::new(),
                }
            }
        }
    }

    fn as_f64(s: &str) -> Option<f64> {
        s.parse::<f64>().ok()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Address(String);

impl Address {
    fn as_f64(&self) -> Option<f64> {
        self.0.parse::<f64>().ok()
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address(value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    String(String),
    Address(Address),
    Number(f64),
    Formula(String),
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        if let Ok(num) = s.parse::<f64>() {
            return Value::Number(num);
        }
        let mut first = s.chars().take(1);
        if first.clone().all(|c| c.is_alphabetic()) {
            return Value::Address(Address(s));
        }
        if first.all(|c| c == '=') {
            return Value::Formula(s);
        }
        Value::String(s)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Value::from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet() {
        let mut ss = Spreadsheet::default();
        assert_eq!(ss.set("a1", "32"), "32");
        assert_eq!(ss.set("a2", "a1"), "32");
        assert_eq!(ss.set("a3", "=a1+a2"), "64");
        assert_eq!(ss.set("a4", "=a1+5"), "37");
    }
}
