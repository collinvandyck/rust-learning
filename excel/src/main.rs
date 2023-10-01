use std::collections::HashMap;

fn main() {
    let mut ss = Spreadsheet::default();
    ss.set("A1", "");
    ss.set("B1", "foobar");
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
            let val = self.value(key);
            println!("{key}={val}");
        }
    }

    fn value(&self, key: &Key) -> String {
        String::from("todo")
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
