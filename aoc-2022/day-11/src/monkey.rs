use core::str;
use std::result;

use crate::prelude::*;

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    items: Vec<Item>,
    op: Op,
}

impl Monkey {
    pub fn load(iter: &mut impl Iterator<Item = String>) -> Option<Self> {
        let idx = Self::parse_monkey(iter.next().unwrap());
        let items = Self::parse_items(iter.next().unwrap());
        let op = Self::parse_operation(iter.next().unwrap());
        iter.next();
        iter.next();
        iter.next();
        dbg!(Some(Self { idx, items, op }))
    }
    fn parse_monkey(input: String) -> usize {
        let parts = &input.split(' ').collect::<Vec<&str>>()[..];
        if let ["Monkey", num] = parts {
            if let Some(num) = num.split(':').next() {
                return num.parse::<usize>().unwrap();
            }
        }
        panic!("invalid monkey: {input}");
    }
    // Starting items: 54, 65, 75, 74
    fn parse_items(input: String) -> Vec<Item> {
        let mut iter = input.trim().split(':');
        if let Some("Starting items") = iter.next() {
            if let Some(nums) = iter.next() {
                return nums
                    .split(",")
                    .map(str::trim)
                    .map(str::parse::<i32>)
                    .map(result::Result::unwrap)
                    .map(Item::new)
                    .collect();
            }
        }
        panic!("invalid items: {input}");
    }
    // Operation: new = old + 6
    fn parse_operation(input: String) -> Op {
        let mut iter = input.trim().split(':');
        if let Some("Operation") = iter.next() {
            if let Some(stmt) = iter.next() {
                let parts = stmt.trim().split(' ').collect::<Vec<&str>>();
                if let ["new", "=", left, op @ ("+" | "*"), right] = parts[..] {
                    return Op::from(left, op, right);
                }
            }
        }
        panic!("invalid operation: {input}")
    }
}
