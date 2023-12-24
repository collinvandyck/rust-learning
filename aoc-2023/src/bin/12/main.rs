#![allow(dead_code, unused)]

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
    groups: Vec<Group>,
    damaged: Vec<usize>,
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
            .collect_vec();
        let damaged = parts
            .next()
            .context("no damaged part")?
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        assert!(parts.next().is_none());
        Ok(Self { groups, damaged })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Group {
    spring: Spring,
    count: usize,
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
                ],
                damaged: vec![1, 1, 3]
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
                ],
                damaged: vec![1, 1, 3]
            })
        );
        Ok(())
    }
}
