#![allow(dead_code, unused)]

use std::fmt::Display;

use itertools::Itertools;
fn main() {}

struct Map {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .map(|row| row.chars().map(Tile::from).collect_vec())
            .collect_vec();
        assert!(tiles.iter().map(|r| r.len()).all_equal());
        let rows = tiles.len();
        let cols = tiles.first().map(|r| r.len()).unwrap_or_default();
        Self {
            rows,
            cols,
            tiles: tiles.concat(),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .chunks(self.cols)
            .map(|r| r.iter().map(|t| t.ch()).collect::<String>())
            .join("\n");
        writeln!(f, "{s}")
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Space,
    Round,
    Cube,
}

impl Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Space,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("bad ch: {ch}"),
        }
    }
    fn ch(&self) -> char {
        match self {
            Tile::Space => '.',
            Tile::Round => 'O',
            Tile::Cube => '#',
        }
    }
}

enum Dir {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let ex1 = include_str!("ex1.txt");
        let map = Map::parse(ex1);
        assert_eq!(map.cols, 10);
        assert_eq!(map.rows, 10);
        assert_eq!(map.tiles.len(), 100);
        assert_eq!(map.to_string(), ex1);
    }
}
