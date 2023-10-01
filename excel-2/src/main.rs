#![allow(dead_code, unused)]
use std::collections::HashMap;

fn main() {}

struct Spreadsheet {
    vals: HashMap<Address, Value>,
}

#[derive(Hash, PartialEq, Clone)]
struct Address(String);

#[derive(Clone)]
struct Value(String);

#[cfg(test)]
mod tests {
    #[test]
    fn test_spreadsheet() {}
}
