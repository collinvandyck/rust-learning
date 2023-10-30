use anyhow::Result;
use std::{fs, result};

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn find(nums: &Vec<i64>, perm_size: usize) -> i64 {
    todo!()
}

struct PermIter<'a, T> {
    items: &'a [T],
    n: usize,
    idxs: Vec<usize>,
}

impl<'a, T> PermIter<'a, T> {
    fn new(items: &'a [T], n: usize) -> Self {
        let mut idxs = vec![];
        for idx in 0..n {
            idxs.push(idx);
        }
        Self { items, n, idxs }
    }
}

impl<'a, T> Iterator for PermIter<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

trait VecExt<'a, T> {
    fn perm_iter(&'a self, n: usize) -> PermIter<'a, T>;
}

impl<'a, T> VecExt<'a, T> for Vec<T> {
    fn perm_iter(&'a self, n: usize) -> PermIter<'a, T> {
        PermIter::new(self, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::VecExt;

    #[test]
    fn test_perm_iter() {
        let items: Vec<i32> = vec![];
        let mut iter = items.perm_iter(0);
        assert_eq!(iter.next(), None);
    }
}

fn part_two() -> Result<()> {
    println!("Part two");
    let nums: Vec<i64> = fs::read_to_string("example.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            for z in &nums[2..] {
                if *x + *y + *z == 2020 {
                    println!("{}", *x * *y * *z);
                }
            }
        }
    }
    let nums: Vec<i64> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            for z in &nums[2..] {
                if *x + *y + *z == 2020 {
                    println!("{}", *x * *y * *z);
                }
            }
        }
    }
    Ok(())
}
fn part_one() -> Result<()> {
    println!("Part one");
    let nums: Vec<i64> = fs::read_to_string("example.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            if *x + *y == 2020 {
                println!("{}", *x * *y);
            }
        }
    }
    let nums: Vec<i64> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            if *x + *y == 2020 {
                println!("{}", *x * *y);
            }
        }
    }
    Ok(())
}
