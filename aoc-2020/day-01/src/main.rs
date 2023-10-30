use anyhow::Result;
use std::{fmt::Debug, fs, result};

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
struct PermIter<T> {
    items: Vec<T>,
    idxs: Vec<usize>,
    closed: bool,
}

impl<T> PermIter<T> {
    fn new(items: Vec<T>, n: usize) -> Self {
        let mut idxs = vec![];
        for idx in 0..n {
            idxs.push(idx);
        }
        let closed = n < items.len() || items.len() == 0;
        Self {
            items,
            idxs,
            closed,
        }
    }
}

impl<T> Iterator for PermIter<T>
where
    T: Clone + Debug,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.closed {
            return None;
        }
        // populate the return vector
        let mut res = vec![];
        for n in 0..self.idxs.len() {
            let idx = self.idxs[n];
            if idx >= self.items.len() {
                return None;
            }
            res.push(self.items[idx].clone())
        }

        let last = self.idxs.len() - 1;
        for n in (0..=last).rev() {
            self.idxs[n] += 1;
            if self.idxs[n] >= self.items.len() {
                if n == 0 {
                    // we're closed.
                    self.closed = true;
                    break;
                }
                self.idxs[n] = 0;
            } else {
                break;
            }
        }
        Some(res)
    }
}

trait VecExt<T> {
    fn perm_iter(&self, n: usize) -> PermIter<T>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: Clone,
{
    fn perm_iter(&self, n: usize) -> PermIter<T> {
        PermIter::new(self.clone(), n)
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

        let items = vec![1];
        let perms = items.perm_iter(1).collect::<Vec<_>>();
        assert_eq!(perms, vec![&[1]]);

        let items = vec![1, 2];
        let perms = items.perm_iter(2).collect::<Vec<_>>();
        assert_eq!(perms, vec![[1, 2], [2, 1], [2, 2]]);
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
