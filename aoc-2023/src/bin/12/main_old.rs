#![allow(dead_code, unused)]

use std::{collections::VecDeque, fmt::Debug};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use strum_macros::EnumIs;
use tracing::{debug, info};

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
        let buf = vec![];
        arrangements(self.springs.as_slice(), self.constraints.as_slice(), buf)
    }
}

// .?..#?# 1,1
fn arrangements(springs: &[Spring], amts: &[usize], mut buf: Vec<Spring>) -> usize {
    if amts.len() == 0 {
        for spring in springs {
            buf.push(*spring);
        }
        let buf = buf.into_iter().map(|s| format!("{s:?}")).join("");
        if springs.iter().any(|s| s.is_damaged()) {
            return 0;
        } else {
            info!("Buf: {buf}");
            return 1;
        }
    }
    let mut acc = 0;
    let mut iter = springs.iter().peekable();
    for (idx, spring) in iter.enumerate() {
        let amt = amts[0];
        match spring {
            Spring::Ok => {
                buf.push(Spring::Ok);
            }
            Spring::Damaged => {
                let ahead = springs[idx..]
                    .iter()
                    .take_while(|s| s.is_unknown() || s.is_damaged())
                    .collect_vec();
                if ahead.len() == amt {
                    let mut buf = buf.clone();
                    for _ in 0..amt {
                        buf.push(Spring::Damaged);
                        let res = arrangements(&springs[idx + amt + 1..], &amts[1..], buf.clone());
                    }
                }
            }
            Spring::Unknown => {}
            Spring::Damaged | Spring::Unknown => {
                let ahead = springs[idx..]
                    .iter()
                    .take_while(|s| {
                        let res = s.is_unknown() || s.is_damaged();
                        res
                    })
                    .count();
                let amt = amts[0];
                match spring {
                    Spring::Damaged => {
                        buf.push(Spring::Damaged);
                    }
                    Spring::Unknown => {
                        buf.push(Spring::Unknown);
                    }
                    _ => {}
                }
                let (has_enough, next): (bool, Option<&Spring>) = if ahead >= amt {
                    let next_idx = idx + amt;
                    if next_idx < springs.len() - 1 {
                        (true, springs.get(next_idx))
                    } else {
                        (true, None)
                    }
                } else {
                    (false, None)
                };
                let recursed = match (has_enough, next) {
                    (false, _) => 0,
                    (true, Some(next)) if next.is_damaged() => 0,
                    (true, Some(next)) => {
                        assert!(next.is_unknown(), "next was: {next:?}");
                        arrangements(&springs[idx + amt + 1..], &amts[1..], buf.clone())
                    }
                    (true, None) => arrangements(&springs[idx + amt..], &amts[1..], buf.clone()),
                };
                acc += recursed;
                if spring.is_damaged() {
                    break;
                }
            }
        }
    }
    acc
}

#[test]
fn test_idx() {
    let nums = [1, 2, 3];
    let nums = &nums[0 + 3..];
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

#[derive(Clone, Copy, PartialEq, Eq, EnumIs)]
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
impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Ok => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

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
    #[traced_test]
    fn test_example_pt1() -> Result<()> {
        let ex1 = include_str!("ex1.txt");
        let records = Records::parse(ex1)?;
        assert_eq!(records.get(5).expect("no record").arrangements(), 10);
        assert_eq!(records.get(0).expect("no record").arrangements(), 1);
        assert_eq!(records.get(1).expect("no record").arrangements(), 4);
        assert_eq!(records.get(2).expect("no record").arrangements(), 1);
        assert_eq!(records.get(3).expect("no record").arrangements(), 1);
        assert_eq!(records.get(4).expect("no record").arrangements(), 4);
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
