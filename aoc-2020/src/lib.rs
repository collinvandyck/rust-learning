use std::{fmt::Debug, fs, path::Path};

use anyhow::Result;

pub fn file_to_lines(p: impl AsRef<Path>) -> Result<Vec<String>> {
    let s = fs::read_to_string(p.as_ref())?;
    let lines = s
        .split("\n")
        .map(ToString::to_string)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    Ok(lines)
}

pub fn combinations<T, I>(items: I, n: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug + Sized,
    I: IntoIterator<Item = T>,
{
    let items: Vec<T> = items.into_iter().collect();
    do_combinations(items, n)
}

// to get combinations where n = 2 and items = a,b,c,d
// a b, a, c, a d, b c, b d, c d
fn do_combinations<T>(items: Vec<T>, n: usize) -> Vec<Vec<T>>
where
    T: Clone + Debug + Sized,
{
    println!("combos items={items:?} n={n}");
    if items.len() < n || n == 0 {
        return vec![vec![]];
    }
    let mut res = vec![];
    for i in 0..(items.len() - n + 1) {
        println!("i={i}");
        let first = items[i].clone();
        let rest = items[i + 1..].to_vec().clone();
        let combos = do_combinations(rest, n - 1);
        println!("  combos={combos:?}");
        for mut combo in combos {
            combo.insert(0, first.clone());
            res.push(combo);
        }
    }
    res
}

#[test]
fn test_combinations() {
    /*
    let res = combinations::<i32>(&[], 0);
    assert!(res.is_empty());
    let res = combinations(&[1], 0);
    assert!(res.is_empty());
    let res = combinations(&[1], 1);
    assert_eq!(res, &[[&1]]);
    let res = combinations(&[1, 2, 3], 1);
    assert_eq!(res, &[[&1], [&2], [&3]]);
    let res = combinations(&[1, 2, 3], 2);
    assert_eq!(res, &[[&1, &2], [&1, &3], [&2, &3]]);
    */
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
