#![allow(dead_code, unused)]

use std::collections::VecDeque;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use strum_macros::EnumIs;

fn main() -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Records(Vec<Record>);

impl Records {
    fn parse(input: &str) -> Result<Self> {
        Ok(Self(
            input
                .lines()
                .map(Record::parse)
                .collect::<std::result::Result<_, _>>()
                .context("parse failure")?,
        ))
    }
}

impl std::ops::Deref for Records {
    type Target = Vec<Record>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Record {
    groups: VecDeque<Group>,
    damaged: VecDeque<usize>,
}

impl Record {
    fn parse(line: &str) -> Result<Self> {
        let mut parts = line.split(" ");
        let groups = parts
            .next()
            .context("no spring part")?
            .chars()
            .map(Spring::from)
            .group_by(|s| *s)
            .into_iter()
            .map(|(spring, xs)| Group::new(spring, xs.count()))
            .collect_vec()
            .into();
        let damaged = parts
            .next()
            .context("no damaged part")?
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec()
            .into();
        assert!(parts.next().is_none());
        Ok(Self { groups, damaged })
    }
    fn arrangements(&self) -> impl Iterator<Item = Record> {
        arrangements(self.clone()).into_iter()
    }
    fn is_complete(&self) -> bool {
        self.damaged.is_empty() && !self.groups.iter().any(|g| g.is_unknown())
    }
}

fn arrangements(rec: Record) -> Vec<Record> {
    let mut res = vec![];
    let mut queue = vec![rec];
    while let Some(rec) = queue.pop() {
        if rec.is_complete() {
            queue.push(rec);
            continue;
        }
        match rec
            .groups
            .iter()
            .enumerate()
            .find(|(idx, group)| group.is_unknown())
        {
            Some((idx, group)) => {
                let count = group.count;
            }
            // if there are no more unknown groups, we're done
            None => break,
        }
    }
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Group {
    spring: Spring,
    count: usize,
}

impl std::ops::Deref for Group {
    type Target = Spring;
    fn deref(&self) -> &Self::Target {
        &self.spring
    }
}

impl Group {
    fn new(spring: Spring, count: usize) -> Self {
        Self { spring, count }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
enum Spring {
    Ok,
    Damaged,
    Unknown,
}

impl Spring {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Ok,
            '?' => Self::Unknown,
            '#' => Self::Damaged,
            _ => panic!("unknown ch: {ch}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_pt1() -> Result<()> {
        let ex1 = include_str!("ex1.txt");
        let records = Records::parse(ex1)?;
        assert_eq!(
            records
                .get(0)
                .expect("no record")
                .arrangements()
                .collect_vec()
                .len(),
            1
        );
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let ex1 = include_str!("ex1.txt");
        let records = Records::parse(ex1)?;
        assert_eq!(records.len(), 6);
        assert_eq!(
            records.get(0),
            Some(&Record {
                groups: vec![
                    Group::new(Spring::Unknown, 3),
                    Group::new(Spring::Ok, 1),
                    Group::new(Spring::Damaged, 3),
                ]
                .into(),
                damaged: vec![1, 1, 3].into()
            })
        );
        assert_eq!(
            records.get(1),
            Some(&Record {
                groups: vec![
                    Group::new(Spring::Ok, 1),
                    Group::new(Spring::Unknown, 2),
                    Group::new(Spring::Ok, 2),
                    Group::new(Spring::Unknown, 2),
                    Group::new(Spring::Ok, 3),
                    Group::new(Spring::Unknown, 1),
                    Group::new(Spring::Damaged, 2),
                    Group::new(Spring::Ok, 1),
                ]
                .into(),
                damaged: vec![1, 1, 3].into(),
            })
        );
        Ok(())
    }
}
