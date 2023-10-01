use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn main() {
    let mut ss = Spreadsheet::default();
    ss.set("A1", "32");
    ss.set("B1", "44");
    ss.set("A2", "A1");
    ss.set("N1", "=A1+B1");
    ss.set("N2", "=A1+B4");
    ss.set("N3", "=N1+B1");
    ss.set("N4", "=N3+1");
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
            let val = self.get(key);
            println!("{key}: {val}");
        }
    }

    fn get(&self, key: &Key) -> String {
        let mut visited = HashSet::default();
        self.eval(key, &mut visited)
    }

    fn eval(&self, key: &Key, visited: &mut HashSet<Key>) -> String {
        if let Ok(v) = key.parse::<i64>() {
            return format!("{v}");
        }
        if visited.contains(key) {
            return self
                .vals
                .get(key)
                .map(|v| v.to_string())
                .unwrap_or_default();
        }
        visited.insert(key.to_string());
        self.vals
            .get(key)
            .map(|val| match val {
                Val::Literal(val) => val.clone(),
                Val::Reference(val) => self.eval(&val, visited),
                Val::Formula(val) => {
                    let parts: Vec<&str> = val.split('+').collect();
                    if let [first, second] = parts[..] {
                        let (first, second) = (first.to_string(), second.to_string());
                        let first = self.eval(&first, visited);
                        let second = self.eval(&second, visited);
                        if let Some(val) = Self::add(first, second) {
                            return val;
                        }
                    }
                    format!("={val}")
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

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Val::Literal(v) => v,
            Val::Reference(v) => v,
            Val::Formula(v) => v,
        };
        write!(f, "{s}")
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spreadsheet() {
        let mut ss = Spreadsheet::default();
        ss.set("A1", "32");
        ss.set("B1", "44");
        ss.set("A2", "A1");
        ss.set("N1", "=A1+B1");
        ss.set("N2", "=A1+B4");
        ss.set("N3", "=N1+B1");
        ss.set("N4", "=N3+1");
        ss.set("N5", "=N5+1");

        let check = |k: &str, v: &str| {
            let res = ss.get(&k.to_string());
            assert_eq!(res, v, "expected {k}={v} but got {res}");
        };

        check("A1", "32");
        check("B1", "44");
        check("A2", "32");
        check("N1", "76");
        check("N2", "=A1+B4"); // b4 does not exist
        check("N3", "120");
        check("N4", "121");
        check("N5", "=N5+1"); // cycle detect
    }
}
