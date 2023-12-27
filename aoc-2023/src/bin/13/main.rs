#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {}

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
    fn vertical_mirrors(&self) -> usize {
        todo!()
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(Pattern::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
