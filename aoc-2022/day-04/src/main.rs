use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let read = BufReader::new(file);
    let mut contains = 0;
    let mut overlaps = 0;
    for line in read.lines() {
        let line = line.unwrap();
        let parts = line.split(',').map(Range::from_str).collect::<Vec<_>>();
        let first = parts.get(0).unwrap();
        let second = parts.get(1).unwrap();
        if first.contains(&second) || second.contains(&first) {
            contains += 1;
        }
        if first.overlaps(&second) {
            overlaps += 1;
        }
    }
    println!("Count: {contains}");
    println!("Overlaps: {overlaps}");
}

#[derive(Debug, Copy, Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split('-');
        let from = iter.next().unwrap().parse::<i32>().unwrap();
        let to = iter.next().unwrap().parse::<i32>().unwrap();
        Self { from, to }
    }
    fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    /*
    5-7,7-9 overlaps in a single section, 7.
    2-8,3-7 overlaps all of the sections 3 through 7.
    6-6,4-6 overlaps in a single section, 6.
    2-6,4-8 overlaps in sections 4, 5, and 6.

    5-7,7-9 -> 0-2,2-4
    2-8,3-7 -> 0-6,1-5
    6-6,4-6 -> 0-2,2-2
    2-6,4-8 -> 0-4,2-6

    1-3,5-9 does not overlap
    0-2,4-8
    */
    fn overlaps(&self, other: &Self) -> bool {
        let (mut one, mut two) = Self::starting(*self, *other);
        let delta = one.from;
        one.from -= delta;
        one.to -= delta;
        two.from -= delta;
        two.to -= delta;
        one.to >= two.from
    }

    fn starting(one: Range, two: Range) -> (Range, Range) {
        if one.from < two.from {
            (one, two)
        } else {
            (two, one)
        }
    }
}

#[test]
fn test_overlaps() {
    assert!(Range { from: 5, to: 7 }.overlaps(&Range { from: 7, to: 9 }));
    assert!(Range { from: 2, to: 8 }.overlaps(&Range { from: 3, to: 7 }));
    assert!(Range { from: 6, to: 6 }.overlaps(&Range { from: 4, to: 6 }));
    assert!(Range { from: 2, to: 6 }.overlaps(&Range { from: 4, to: 8 }));
}
