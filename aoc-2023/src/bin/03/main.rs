#![allow(dead_code, unused)]

use std::collections::HashMap;

fn main() {
    let ex = include_str!("example.txt");
    println!("p1ex={}", sum_of_part_numbers(ex));
}

fn sum_of_part_numbers(input: &str) -> u64 {
    Schema::new(input).parts().iter().map(|s| s.num).sum()
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIs)]
enum Value {
    Space,
    Digit(u32),
    Symbol(char),
}

impl Value {
    fn from(ch: char) -> Self {
        match (ch, ch.to_digit(10)) {
            (_, Some(v)) => Value::Digit(v),
            ('.', _) => Value::Space,
            _ => Value::Symbol(ch),
        }
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
    fn height(&self) -> usize {
        self.0.len()
    }
    fn width(&self) -> usize {
        self.0.get(0).map(|l| l.len()).unwrap_or_default()
    }
    fn get(&self, x: usize, y: usize) -> Option<Value> {
        self.0.get(y).map(|row| row.get(x)).flatten().copied()
    }
    fn parts(&self) -> Vec<Part> {
        let mut parts = vec![];
        for (y, row) in self.0.iter().enumerate() {
            println!("Got row: {row:?}");
            let mut iter = row.iter().enumerate();
            loop {
                let digits = iter
                    .by_ref()
                    .take_while(|v| v.1.is_digit())
                    .collect::<Vec<_>>();
                if !digits.is_empty() {
                    println!("Digits: {digits:?}");
                    continue;
                }
                // todo: check for space and for symbol
                break;
            }
        }
        parts
    }
}
