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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Record {
    springs: Vec<Spring>,
    constraints: Vec<usize>,
}

impl Record {
    fn new<S, C>(s: S, c: C) -> Self
    where
        S: IntoIterator<Item = Spring>,
        C: IntoIterator<Item = usize>,
    {
        let springs = s.into_iter().collect();
        let constraints = c.into_iter().collect_vec().into();
        Self {
            springs,
            constraints,
        }
    }
    fn parse(line: &str) -> Result<Self> {
        let mut parts = line.split(" ");
        let springs = parts
            .next()
            .context("no spring part")?
            .chars()
            .map(Spring::from)
            .collect::<Vec<_>>();
        let constraints = if let Some(part) = parts.next() {
            part.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
        } else {
            vec![]
        };
        assert!(parts.next().is_none());
        Ok(Self::new(springs, constraints))
    }
    fn arrangements(&self) -> usize {
        let mut res = 0;
        arrangements(
            self.springs.as_slice(),
            self.constraints.as_slice(),
            &mut res,
        );
        res
    }
}

fn arrangements(springs: &[Spring], amts: &[usize], res: &mut usize) {}

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
    fn test_simple() -> Result<()> {
        // an empty record
        let rec = Record::parse("")?;
        assert_eq!(rec, Record::new(vec![], vec![]));
        let arrs = rec.arrangements();
        assert_eq!(arrs, 1);

        // a record with one ok spring
        let rec = Record::parse(".")?;
        assert_eq!(rec, Record::new(vec![Spring::Ok], vec![]));
        let arrs = rec.arrangements();
        assert_eq!(arrs, 1);

        // a record with one ok, and one damaged
        let rec = Record::parse(".#")?;
        assert_eq!(rec, Record::new(vec![Spring::Ok, Spring::Damaged], vec![]));
        let arrs = rec.arrangements();
        assert_eq!(arrs, 1);

        // a record with no springs and a constraints
        let rec = Record::parse(" 1")?;
        assert_eq!(rec, Record::new(vec![], vec![1]));
        let arrs = rec.arrangements();
        // no constraints -- was not solved
        assert_eq!(arrs, 0);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_example_pt1() -> Result<()> {
        let ex1 = include_str!("ex1.txt");
        let records = Records::parse(ex1)?;
        assert_eq!(records.get(0).expect("no record").arrangements(), 1);
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
                springs: vec![
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Ok,
                    Spring::Damaged,
                    Spring::Damaged,
                    Spring::Damaged,
                ],
                constraints: vec![1, 1, 3].into()
            })
        );
        Ok(())
    }
}
