#![allow(dead_code, unused)]

use std::{
    iter::{self, Copied, Rev},
    slice,
};
fn main() {
    let example = include_str!("example.txt");
    println!("p1ex={}", sum_nexts(example));
}

fn sum_nexts(input: &str) -> u64 {
    parse(input).into_iter().map(next_value).sum()
}

fn next_value(seq: Seq) -> u64 {
    let nums: Vec<u64> = seq.diffs().collect();
    todo!()
}

#[test]
fn test_next_value() {
    let seqs = parse("0   3   6   9  12  15");
    assert_eq!(seqs.len(), 1);
    let seq = seqs.into_iter().nth(0).unwrap();
    assert_eq!(next_value(seq), 18);
}

#[test]
fn test_diff_iter() {
    let seq = parse_seq("0 3 6 9 12 15");
    let dif: Vec<u64> = seq.diffs().collect();
    assert_eq!(dif, &[3, 3, 3, 3, 3]);
    let dif: Vec<u64> = Seq(dif).diffs().collect();
    assert_eq!(dif, &[0, 0, 0, 0]);
}

#[derive(Debug, Clone)]
struct Seq(Vec<u64>);

impl Seq {
    fn diffs<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        DiffIter::from_slice(&self.0)
    }

    fn diffs_seq(&self) -> Seq {
        Seq(self.diffs().collect())
    }
}

struct DiffIter<T>
where
    T: Iterator<Item = u64>,
{
    iter: T,
    last: Option<u64>,
}

impl<T> DiffIter<iter::Rev<T>>
where
    T: Iterator<Item = u64> + DoubleEndedIterator,
{
    fn from_iter(iter: T) -> Self {
        let mut iter = iter.rev();
        let last = iter.next();
        Self { iter, last }
    }
}

impl<'a> DiffIter<Box<dyn Iterator<Item = u64> + 'a>> {
    fn from_slice_with_alloc(s: &'a [u64]) -> Self {
        let iter = s.iter().rev().copied();
        let mut iter = Box::new(iter);
        let last = iter.next();
        Self { iter, last }
    }
}

impl<'a> DiffIter<Copied<Rev<slice::Iter<'a, u64>>>> {
    fn from_slice(s: &'a [u64]) -> Self {
        let mut iter = s.iter().rev().copied();
        let last = iter.next();
        Self { iter, last }
    }
}

impl<T> Iterator for DiffIter<T>
where
    T: Iterator<Item = u64>,
{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            Some(last) => match self.iter.next() {
                Some(next) => {
                    let diff = last.saturating_sub(next);
                    self.last = Some(next);
                    Some(diff)
                }
                None => {
                    self.last = None;
                    None
                }
            },
            None => None,
        }
    }
}

fn parse(input: &str) -> Vec<Seq> {
    input.lines().map(parse_seq).collect()
}

fn parse_seq(input: &str) -> Seq {
    Seq(input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect())
}
