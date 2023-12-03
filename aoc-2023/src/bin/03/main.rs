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
    Digit(u64),
    Symbol(char),
}

impl Value {
    fn from(ch: char) -> Self {
        match (ch, ch.to_digit(10)) {
            (_, Some(v)) => Value::Digit(v.into()),
            ('.', _) => Value::Space,
            _ => Value::Symbol(ch),
        }
    }
}

#[derive(Debug)]
struct Part {
    num: u64,
    ul: Point,
    lr: Point,
}

#[derive(Debug)]
struct Point(usize, usize);
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
            let mut part: Option<Part> = None;
            for (x, v) in row.iter().enumerate() {
                match part.as_mut() {
                    Some(p) => {
                        if let Value::Digit(v) = v {
                            p.num = p.num * 10 + v;
                            p.lr = Point(x, y);
                        } else {
                            parts.push(part.take().unwrap());
                        }
                    }
                    None => {
                        if let Value::Digit(v) = v {
                            part = Some(Part {
                                num: *v,
                                ul: Point(x, y),
                                lr: Point(x, y),
                            })
                        }
                    }
                }
            }
            if let Some(part) = part.take() {
                parts.push(part);
            }
        }
        parts
    }
}
