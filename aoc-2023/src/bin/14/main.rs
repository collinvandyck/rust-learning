#![allow(dead_code, unused)]

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
    }
}
