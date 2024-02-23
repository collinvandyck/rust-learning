#![allow(dead_code, unused)]

use itertools::Itertools;
use std::fmt::Display;

fn main() {
    let ex = include_str!("ex1.txt");
    let map = Map::parse(ex);
    println!("{map}");
    assert_eq!(map.get(0, 0), Some(Tile::from('2')));
    assert_eq!(map.get(0, 1), Some(Tile::from('4')));
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile {
    ch: char,
    val: u32,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        Self {
            ch,
            val: ch.to_digit(10).unwrap(),
        }
    }
}

impl Map {
    fn parse(s: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.trim().chars().map(Into::into).collect())
            .collect();
        assert!(tiles.iter().map(|v| v.len()).all_equal());
        Self { tiles }
    }
    fn get(&self, row: usize, col: usize) -> Option<Tile> {
        self.tiles.get(row).and_then(|v| v.get(col)).copied()
    }
    fn cols(&self) -> usize {
        self.tiles.first().map(|l| l.len()).unwrap_or_default()
    }
    fn rows(&self) -> usize {
        self.tiles.len()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .iter()
            .map(|r| r.iter().map(|t| t.ch).collect::<String>())
            .join("\n");
        write!(f, "{s}")
    }
}
