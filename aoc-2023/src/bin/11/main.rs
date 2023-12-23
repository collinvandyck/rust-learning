#![allow(unused, dead_code)]

use anyhow::Result;
use itertools::Itertools;
use std::{fmt::Display, ops::Deref};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let map = Map::parse(ex1);
    map.galaxy_pairs()
        .into_iter()
        .for_each(|(t1, t2)| println!("{t1:?} {t2:?}"));
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn parse(input: &str) -> Self {
        Self(
            input
                .trim()
                .lines()
                .enumerate()
                .map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .map(|(x, ch)| Tile::new(x, y, Glyph::from(ch)))
                        .collect()
                })
                .collect(),
        )
    }
    fn expand(&mut self) {
        (0..self.num_rows())
            .into_iter()
            .filter(|y| self.row_iter(*y).all(|t| t.is_space()))
            .enumerate()
            .map(|(c, y)| c + y)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|idx| self.expand_row(idx));
        (0..self.num_cols())
            .into_iter()
            .filter(|x| self.col_iter(*x).all(|t| t.is_space()))
            .enumerate()
            .map(|(c, x)| c + x)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|idx| self.expand_col(idx));
        self.0.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, tile)| {
                tile.x = x;
                tile.y = y;
            })
        })
    }
    fn expand_row(&mut self, y: usize) {
        self.0.insert(
            y,
            (0..self.num_cols())
                .into_iter()
                .map(|x| Tile::new(x, y, Glyph::Space))
                .collect_vec(),
        );
    }
    fn expand_col(&mut self, x: usize) {
        self.0
            .iter_mut()
            .enumerate()
            .for_each(|(y, row)| row.insert(x, Tile::new(x, y, Glyph::Space)))
    }
    fn galaxy_pairs(&self) -> Vec<(&Tile, &Tile)> {
        self.galaxy_iter()
            .combinations(2)
            .map(|v| (v[0], v[1]))
            .collect()
    }
    fn galaxy_iter(&self) -> impl Iterator<Item = &Tile> {
        self.0
            .iter()
            .flat_map(|row| row.into_iter().filter(|tile| tile.is_galaxy()))
    }
    fn row_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.0
            .get(idx)
            .map(|s| s.as_slice())
            .unwrap_or(&[])
            .into_iter()
    }
    fn col_iter(&self, idx: usize) -> impl Iterator<Item = &Tile> {
        self.0.iter().filter_map(move |row| row.get(idx))
    }
    fn num_rows(&self) -> usize {
        self.0.len()
    }
    fn num_cols(&self) -> usize {
        self.0.get(0).map(|v| v.len()).unwrap_or_default()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|row| row.iter().map(|t| t.ch()).collect::<String>())
            .join("\n");
        write!(f, "{s}\n")
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    glyph: Glyph,
    x: usize,
    y: usize,
}

impl Tile {
    fn new(x: usize, y: usize, glyph: Glyph) -> Self {
        Self { x, y, glyph }
    }
}

impl Deref for Tile {
    type Target = Glyph;
    fn deref(&self) -> &Self::Target {
        &self.glyph
    }
}

#[derive(Debug, Clone, Copy, strum_macros::EnumIs)]
enum Glyph {
    Galaxy,
    Space,
}

impl Glyph {
    fn ch(&self) -> char {
        match self {
            Self::Galaxy => '#',
            Self::Space => '.',
        }
    }
}

impl From<char> for Glyph {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Galaxy,
            '.' => Self::Space,
            _ => panic!("unknown char: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_pairs() {
        let input = include_str!("ex1.txt");
        let map = Map::parse(input);
        let pairs = map.galaxy_pairs().into_iter().count();
        assert_eq!(pairs, 36);
    }

    #[test]
    fn test_map_serde() {
        let input = include_str!("ex1.txt");
        let map = Map::parse(input);
        let map_str = map.to_string();
        assert_eq!(map_str, input);
    }

    #[test]
    fn test_map_expand() {
        let ex1 = include_str!("ex1.txt");
        let mut map = Map::parse(ex1);
        map.expand();
        let map_str = map.to_string();
        let ex1_exp = include_str!("ex1-exp.txt");
        assert_eq!(map_str, ex1_exp);
    }
}
