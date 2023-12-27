#![allow(dead_code, unused)]

use itertools::Itertools;
use std::{cmp::Ordering, collections::VecDeque, thread, time::Duration};
use tracing::{debug, info};

fn main() {
    tracing_subscriber::fmt().init();
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1={}", sum_of_arrangements(ex1, false));
    println!("p1in1={}", sum_of_arrangements(in1, false));
    println!("p2ex1={}", sum_of_arrangements(ex1, true));
    //println!("p2in1={}", sum_of_arrangements(in1, true));
}

fn sum_of_arrangements(input: &str, inflate: bool) -> usize {
    parse(input)
        .into_iter()
        .map(|r| if inflate { r.inflate() } else { r })
        .map(|r| r.combinations())
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Record {
    springs: Vec<char>,
    dmgs: VecDeque<usize>,
}

#[derive(Debug, Clone)]
struct Solver {
    record: Record,
    idx: usize, // pos into the record springs
}

impl Solver {
    fn new(record: Record) -> Self {
        Solver { record, idx: 0 }
    }
    fn bump(mut self, amt: usize) -> Self {
        self.idx += amt;
        self
    }
    fn replace(mut self, ch: char) -> Self {
        self.record.springs[self.idx] = ch;
        self
    }
    fn replace_bump(mut self, ch: char, amt: usize) -> Self {
        for i in self.idx..self.idx + amt {
            self.record.springs[i] = ch;
        }
        self.idx += amt;
        self
    }
    fn pop_amt(mut self) -> Self {
        self.record.dmgs.pop_front();
        self
    }
    fn remaining(&self) -> &[char] {
        &self.record.springs[self.idx..]
    }
}

impl std::ops::Deref for Solver {
    type Target = Record;
    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl Record {
    fn parse(input: &str) -> Self {
        let mut parts = input.splitn(2, " ");
        let springs = parts.next().expect("no springs").chars().collect();
        let dmgs = parts
            .next()
            .expect("no dmgs")
            .split(",")
            .map(|n| n.parse::<usize>().expect("parse fail"))
            .collect();
        Self { springs, dmgs }
    }
    fn inflate(self) -> Self {
        let springs = (0..5)
            .map(|_| self.springs.clone())
            .reduce(|o1, o2| [o1, vec!['?'], o2].concat())
            .unwrap();
        let dmgs = (0..5)
            .map(|_| self.dmgs.clone())
            .fold(vec![], |mut acc, mut f| {
                acc.extend(f);
                acc
            })
            .into();
        Self { springs, dmgs }
    }
    fn combinations(&self) -> usize {
        info!("combinations: {:?}", self.springs.iter().join(""));
        // initialize the queue and initial work
        let mut res: Vec<Record> = vec![];
        let mut queue: VecDeque<Solver> = vec![].into();
        queue.push_front(Solver::new(self.clone()));

        while let Some(solver) = queue.pop_front() {
            let remain = solver.remaining();
            debug!("Loop {}", remain.iter().collect::<String>());
            thread::sleep(Duration::from_millis(20 * 0));
            if remain.is_empty() {
                if solver.dmgs.is_empty() {
                    res.push(solver.record);
                }
                continue;
            }
            if solver.record.dmgs.is_empty() {
                if remain.iter().all(|c| !c.is_damaged()) {
                    res.push(solver.record);
                }
                continue;
            }
            match &remain[..] {
                &['.', ..] => queue.push_front(solver.bump(1)),
                &['?', ..] => {
                    queue.push_front(solver.clone().replace_bump('.', 1));
                    queue.push_front(solver.replace('#'));
                }
                &['#', ..] => {
                    let amt: usize = solver.record.dmgs[0];
                    let nexts = remain
                        .iter()
                        .take_while(|c| c.is_unknown() || c.is_damaged())
                        .collect_vec();
                    debug!("  remain: {remain:?}");
                    debug!("  nexts:  {nexts:?}");
                    debug!("  amt   : {amt}");
                    match nexts.len().cmp(&amt) {
                        Ordering::Equal => {
                            // perfect fit
                            queue.push_front(solver.replace_bump('#', amt).pop_amt());
                        }
                        Ordering::Greater => {
                            if !remain[amt].is_damaged() {
                                // then we can move on
                                queue.push_front(
                                    solver.replace_bump('#', amt).replace_bump('.', 1).pop_amt(),
                                );
                            }
                        }
                        _ => {}
                    }
                }
                _ => panic!("unhandled solver: {solver:?} \n remain: {remain:?}"),
            }
        }
        res.len()
    }

    fn is_done(&self) -> bool {
        self.dmgs.is_empty()
    }

    fn is_ok(&self) -> bool {
        self.springs.iter().all(|c| c.is_ok() || c.is_unknown())
    }
}

trait Spring {
    fn is_ok(&self) -> bool;
    fn is_unknown(&self) -> bool;
    fn is_damaged(&self) -> bool;
}

impl Spring for char {
    fn is_ok(&self) -> bool {
        self == &'.'
    }
    fn is_unknown(&self) -> bool {
        self == &'?'
    }
    fn is_damaged(&self) -> bool {
        self == &'#'
    }
}

fn parse(input: &str) -> Vec<Record> {
    input.lines().map(Record::parse).collect()
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn test_ex2() {
        let mut recs: Vec<Record> = parse(include_str!("ex1.txt"))
            .into_iter()
            .map(|r| r.inflate())
            .collect_vec();
        assert_eq!(recs[0].combinations(), 1);
        assert_eq!(recs[1].combinations(), 16384);
        assert_eq!(recs[2].combinations(), 1);
        assert_eq!(recs[3].combinations(), 16);
        assert_eq!(recs[4].combinations(), 2500);
        assert_eq!(recs[5].combinations(), 506250);
    }

    #[test]
    fn test_ex1() {
        let recs = parse(include_str!("ex1.txt"));
        assert_eq!(recs[0].combinations(), 1);
        assert_eq!(recs[1].combinations(), 4);
        assert_eq!(recs[2].combinations(), 1);
        assert_eq!(recs[3].combinations(), 1);
        assert_eq!(recs[4].combinations(), 4);
        assert_eq!(recs[5].combinations(), 10);
    }

    #[test]
    fn test_pt1() {
        let ex1 = include_str!("ex1.txt");
        assert_eq!(sum_of_arrangements(ex1, false), 21);
        let in1 = include_str!("in1.txt");
        assert_eq!(sum_of_arrangements(in1, false), 7251);
    }

    #[test]
    fn test_inflate() {
        let mut rec = Record::parse(".# 1").inflate();
        assert_eq!(rec, Record::parse(".#?.#?.#?.#?.# 1,1,1,1,1"));
    }

    #[test]
    fn test_parse() {
        let recs = parse(include_str!("ex1.txt"));
        assert_eq!(
            recs[0],
            Record {
                springs: "???.###".chars().collect(),
                dmgs: vec![1, 1, 3].into(),
            }
        )
    }
}
