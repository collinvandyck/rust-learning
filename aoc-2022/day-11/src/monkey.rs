use core::str;
use std::result;

use crate::prelude::*;

#[derive(Debug)]
pub struct Monkey {
    idx: usize,
    items: Vec<Item>,
    op: Op,
    test: Test,
}

pub struct SendTo {
    pub idx: usize,
    pub item: Item,
}

impl Monkey {
    pub fn inspect(&mut self) -> Vec<SendTo> {
        self.items
            .drain(..)
            .map(|mut item| {
                item.inspect(&self.op);
                let idx = self.test.evaluate(&item);
                SendTo { idx, item }
            })
            .collect()
    }
}

// things to load monkey with
impl Monkey {
    pub fn load(iter: &mut impl Iterator<Item = String>) -> Option<Self> {
        let idx = Self::parse_monkey(iter.next().unwrap().as_str());
        let items = Self::parse_items(iter.next().unwrap().as_str());
        let op = Self::parse_operation(iter.next().unwrap().as_str());
        let test = Self::parse_test(iter);
        Some(Self {
            idx,
            items,
            op,
            test,
        })
    }
    fn parse_monkey(input: &str) -> usize {
        let parts = &input.split(' ').collect::<Vec<&str>>()[..];
        if let ["Monkey", num] = parts {
            if let Some(num) = num.split(':').next() {
                return num.parse::<usize>().unwrap();
            }
        }
        panic!("invalid monkey: {input}");
    }
    // Starting items: 54, 65, 75, 74
    fn parse_items(input: &str) -> Vec<Item> {
        let mut iter = input.trim().split(':');
        if let Some("Starting items") = iter.next() {
            if let Some(nums) = iter.next() {
                return nums
                    .split(',')
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
    fn parse_operation(input: &str) -> Op {
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
    // Test: divisible by 19
    //  If true: throw to monkey 2
    //  If false: throw to monkey 0
    fn parse_test(iter: &mut impl Iterator<Item = String>) -> Test {
        let line = iter.next().unwrap();
        let line = line.trim().split(' ').collect::<Vec<&str>>();
        if let ["Test:", "divisible", "by", num] = line[..] {
            let divisible_by = num.parse::<i32>().unwrap();
            let line = iter.next().unwrap();
            let line = line.trim().split(' ').collect::<Vec<&str>>();
            if let ["If", "true:", "throw", "to", "monkey", num] = line[..] {
                let throw_to_true = num.parse::<usize>().unwrap();
                let line = iter.next().unwrap();
                let line = line.trim().split(' ').collect::<Vec<&str>>();
                if let ["If", "false:", "throw", "to", "monkey", num] = line[..] {
                    let throw_to_false = num.parse::<usize>().unwrap();
                    return Test::new(divisible_by, throw_to_true, throw_to_false);
                }
            }
        }
        panic!("Invalid test!")
    }
}
