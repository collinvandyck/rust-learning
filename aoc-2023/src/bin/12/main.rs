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
    springs: Vec<Spring>,
    constraints: VecDeque<usize>,
}

impl Record {
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
        }
        .into();
        assert!(parts.next().is_none());
        Ok(Self {
            springs,
            constraints,
        })
    }
    fn arrangements(&self) -> impl Iterator<Item = Record> {
        arrangements(self.clone()).into_iter()
    }
}

fn arrangements(rec: Record) -> Vec<Record> {
    let mut res = vec![];
    let mut queue = vec![rec];
    while let Some(mut rec) = queue.pop() {
        // if the rec is done, move it to the result vec.
        if rec.constraints.is_empty() && !rec.springs.iter().any(|s| s.is_unknown()) {
            res.push(rec);
            continue;
        }
        let Some(dmg_grp) = rec.constraints.pop_front() else {
            continue;
        };
        // we now have a first group of unknown springs (unk_grp) and a number of damaged springs
        // (dmg_grp) that we must find fits for.
        //
        // if the len of the unknown group is less than the len of the dmg group then it is not
        // possible to move forward.
        //
        // if the len of the uknown group is equal to the len of the dmg group, then there is one
        // option for assigning this dmg. we must verify that the group following the current
        // unknown group does not violate the dmg spec.
        //
        // if the len of the unknown group is greater than the len of the dmg group then we know
        // that if we place the dmg group first, as we must, at that point we know we have to place
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
    fn test_simple() -> Result<()> {
        let rec = Record::parse("")?;
        Ok(())
    }

    #[test]
    #[ignore]
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
