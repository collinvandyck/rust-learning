#![allow(dead_code, unused)]

use itertools::Itertools;
use strum_macros::EnumIs;

fn main() {
    println!("hi");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Records(Vec<Record>);

impl Records {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(Record::parse).collect())
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
    fn parse(line: &str) -> Self {
        let mut parts = line.split(" ");
        let springs = parts
            .next()
            .unwrap()
            .chars()
            .map(Spring::from)
            .collect::<Vec<_>>();
        let groups = springs
            .into_iter()
            .group_by(|spring| *spring)
            .into_iter()
            .map(|(spring, xs)| Group::new(spring, xs.count()))
            .collect_vec();
        let damaged = parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self { groups, damaged }
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
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let records = Records::parse(ex1);
        assert_eq!(records.len(), 6);
        assert_eq!(
            records.get(0),
            Some(&Record {
                groups: vec![
                    Group {
                        spring: Spring::Unknown,
                        count: 3
                    },
                    Group {
                        spring: Spring::Ok,
                        count: 1
                    },
                    Group {
                        spring: Spring::Damaged,
                        count: 3
                    },
                ],
                damaged: vec![1, 1, 3]
            })
        )
    }
}
