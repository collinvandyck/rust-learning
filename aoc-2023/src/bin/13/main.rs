#![allow(dead_code, unused)]

use itertools::Itertools;
use tracing::info;

fn main() {
    let ex1 = include_str!("ex1.txt");
    println!("p1ex1 = {}", summarize_patterns(ex1));
}

fn summarize_patterns(input: &str) -> usize {
    parse(input).iter().map(|p| p.mirrors()).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    ch: char,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    cells: Vec<Vec<Cell>>,
    rows: Vec<Stripe>,
    cols: Vec<Stripe>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Stripe {
    chs: Vec<char>,
}

impl std::ops::Deref for Stripe {
    type Target = Vec<char>;
    fn deref(&self) -> &Self::Target {
        &self.chs
    }
}

impl Stripe {
    fn new(chs: impl Iterator<Item = char>) -> Self {
        let chs = chs.into_iter().collect_vec();
        Self { chs }
    }
}

impl Pattern {
    fn parse(input: &str) -> Pattern {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .into_iter()
                    .enumerate()
                    .map(move |(x, ch)| Cell { ch, x, y })
                    .collect_vec()
            })
            .collect_vec();
        let rows = cells
            .iter()
            .map(|row| row.iter().map(|c| c.ch))
            .map(Stripe::new)
            .collect_vec();
        let cols = (0..cells.first().map(|r| r.len()).unwrap_or_default())
            .map(|x| {
                cells
                    .iter()
                    .map(move |row| row.get(x).map(|c| c.ch).expect("bad col"))
            })
            .map(|s| Stripe::new(s))
            .collect_vec();
        Pattern { cells, rows, cols }
    }
    fn mirrors(&self) -> usize {
        info!("Mirrors");
        fn stripe_reflects(stripes: &[Stripe]) -> usize {
            (1..stripes.len())
                .map(|idx| {
                    let prev = stripes[0..idx].iter().rev();
                    let next = stripes[idx..].iter();
                    let iter = prev.zip(next).take_while(|(a, b)| a == b);
                    iter.clone().for_each(|(s1, s2)| {
                        let s1 = s1.chs.iter().collect::<String>();
                        let s2 = s2.chs.iter().collect::<String>();
                        info!("{s1} {s2}");
                    });
                    iter.count()
                })
                .sum()
        }
        let hrz = stripe_reflects(&self.rows);
        info!("hrz: {hrz}");
        let vrt = stripe_reflects(&self.cols);
        info!("vrt: {vrt}");
        hrz * 100 + vrt
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_ex1_mirrors() {
        let ex1 = include_str!("ex1.txt");
        let pats = parse(ex1);
        let mrs = pats[1].mirrors();
        assert_eq!(mrs, 1);
    }

    #[test]
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let pat = parse(ex1);
        assert_eq!(pat.len(), 2);
        let row: String = pat[0].rows[0].iter().collect();
        assert_eq!(row, "#.##..##.");
        let col: String = pat[0].cols[0].iter().collect();
        assert_eq!(col, "#.##..#");
        let row: String = pat[0].rows.last().expect("last").iter().collect();
        assert_eq!(row, "#.#.##.#.");
        let col: String = pat[0].cols.last().expect("last").iter().collect();
        assert_eq!(col, "..##...");
    }
}
