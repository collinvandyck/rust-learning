pub mod sets;
pub mod prelude {
    pub use super::*;
    pub use anyhow::anyhow;
    pub use anyhow::bail;
    pub use anyhow::Result;
    pub use once_cell::sync::Lazy;
    pub use regex::Regex;
    pub use std::collections::HashMap;
    pub use std::collections::HashSet;
    pub use std::path::PathBuf;
    pub use std::result::Result as StdResult;
    pub use std::str::FromStr;
    pub use std::{fmt::Debug, fs, path::Path};
}
use prelude::*;
use std::{fs::File, io::BufReader};

pub trait VecExt {
    type Item;
    fn grouped<F>(self, f: F) -> Vec<Vec<Self::Item>>
    where
        F: Fn(&Self::Item) -> bool;
}

impl<T> VecExt for Vec<T> {
    type Item = T;
    fn grouped<F>(self, f: F) -> Vec<Vec<Self::Item>>
    where
        F: Fn(&Self::Item) -> bool,
    {
        groups(self, f)
    }
}

pub fn groups<T, I, F>(items: I, test: F) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T>,
    F: Fn(&T) -> bool,
{
    let mut res = vec![];
    let mut this = vec![];
    for item in items {
        if test(&item) {
            if !this.is_empty() {
                res.push(this);
                this = vec![];
            }
        } else {
            this.push(item);
        }
    }
    if !this.is_empty() {
        res.push(this);
    }
    res
}

pub fn file_to_lines(p: impl AsRef<Path>) -> Result<Vec<String>> {
    use std::io::BufRead;
    let reader = BufReader::new(File::open(p.as_ref())?);
    Ok(reader.lines().collect::<StdResult<Vec<_>, _>>()?)
}

pub fn combinations<T, I>(items: I, n: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug + Sized,
    I: IntoIterator<Item = T>,
{
    let items: Vec<T> = items.into_iter().collect();
    do_combinations(items, n)
}

fn do_combinations<T>(items: Vec<T>, n: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug + Sized,
{
    if items.len() < n || n == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    for i in 0..(items.len() - n + 1) {
        let first = items[i].clone();
        let rest = items[i + 1..].to_vec().clone();
        let combos = do_combinations(rest, n - 1);
        for mut combo in combos {
            combo.insert(0, first.clone());
            res.push(combo);
        }
    }
    res
}

#[test]
fn test_combinations() {
    assert_eq!(combinations(vec![0_i32; 0], 0), vec![vec![]],);
    assert_eq!(combinations([1, 2, 3], 0), vec![vec![]],);
    assert_eq!(
        combinations([1, 2, 3], 2),
        vec![vec![1, 2], vec![1, 3], vec![2, 3,]]
    );
    assert_eq!(
        combinations([1, 2, 3, 4], 3),
        vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]]
    );
    assert_eq!(combinations([1, 2, 3, 4], 4), vec![vec![1, 2, 3, 4]],);
}
