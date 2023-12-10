#![allow(dead_code, unused)]

use itertools::Itertools;
use std::{
    iter::{self, Copied, Rev},
    slice,
};

type Num = i64;

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", sum_nexts(example, true));
    println!("p1in={}", sum_nexts(input, true));
}

fn sum_nexts(input: &str, forward: bool) -> Num {
    parse(input)
        .into_iter()
        .map(|nums| next(nums.clone(), forward))
        .sum()
}

fn next(nums: Vec<Num>, forward: bool) -> Num {
    let mut stack = vec![];
    stack.push(nums);
    while !stack_done(&stack) {
        let last = stack.last().unwrap();
        let next = last
            .windows(2)
            .map(|s| (s[0], s[1]))
            .map(|(one, two)| two - one)
            .collect::<Vec<_>>();
        stack.push(next);
    }
    stack.reverse();
    stack
        .iter()
        .flat_map(|f| f.last())
        .copied()
        .reduce(|a, b| a + b)
        .unwrap_or_default()
}

fn stack_done(stack: &Vec<Vec<Num>>) -> bool {
    stack
        .last()
        .map(|s| s.iter().all(|n| n == &0))
        .unwrap_or_default()
}

#[test]
fn test_next() {
    // test forward directions
    let nums = vec![0, 3, 6, 9, 12, 15];
    assert_eq!(next(nums, true), 18);
    let nums = vec![1, 3, 6, 10, 15, 21];
    assert_eq!(next(nums, true), 28);
}

fn parse(input: &str) -> Vec<Vec<Num>> {
    input.lines().map(parse_seq).collect()
}

fn parse_seq(input: &str) -> Vec<Num> {
    input
        .split_whitespace()
        .map(|s| s.parse::<Num>().unwrap())
        .collect()
}
