#![allow(dead_code, unused)]

use itertools::Itertools;
use std::fmt::Display;

fn main() {
    let ex = include_str!("ex1.txt");
    let map = Map::parse(ex);
    println!("{map}");
}

struct Map {
    tiles: Vec<Vec<char>>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let tiles: Vec<Vec<char>> = s.lines().map(|l| l.trim().chars().collect()).collect();
        assert!(tiles.iter().map(|v| v.len()).all_equal());
        Self { tiles }
    }
    fn get(&self, row: usize, col: usize) -> Option<char> {
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
            .map(|r| r.iter().collect::<String>())
            .join("\n");
        write!(f, "{s}")
    }
}
