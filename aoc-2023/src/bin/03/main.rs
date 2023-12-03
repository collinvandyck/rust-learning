#![allow(dead_code, unused)]

use std::collections::HashMap;

fn main() {
    let ex = include_str!("example.txt");
    println!("p1ex={}", sum_of_part_numbers(ex));
}

fn sum_of_part_numbers(input: &str) -> u64 {
    Schema::new(input).parts().iter().map(|s| s.num).sum()
}

enum Value {
    Space,
}

impl Value {
    fn from(ch: char) -> Self {
        todo!()
    }
}

struct Part {
    num: u64,
}

struct Schema(Vec<Vec<Value>>);

impl Schema {
    fn new(s: &str) -> Self {
        Self(
            s.lines()
                .map(|l| l.chars().map(Value::from).collect())
                .collect(),
        )
    }
    fn parts(&self) -> Vec<Part> {
        todo!()
    }
}
