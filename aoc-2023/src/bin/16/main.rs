#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    tiles: Vec<TileXY>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileXY {
    tile: Tile,
    point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum Tile {
    Space,    // .
    MirRight, // /
    MirLeft,  // \
    SplitV,   // |
    SplitH,   // -
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, ch)| {
                        let tile = Tile::from_ch(ch);
                        let point = Point::new(x, y);
                        TileXY { tile, point }
                    })
                    .collect_vec()
            })
            .collect_vec();
        let rows = tiles.len();
        let cols = tiles.first().map(|r| r.len()).unwrap_or_default();
        Self {
            tiles: tiles.concat(),
            rows,
            cols,
        }
    }
    fn get(&self, pt: Point) -> Option<&TileXY> {
        let idx = self.idx(pt);
        self.tiles.get(idx)
    }
    fn idx(&self, pt: Point) -> usize {
        pt.y * self.cols + pt.x
    }
}

impl Tile {
    fn from_ch(ch: char) -> Self {
        match ch {
            '.' => Self::Space,
            '/' => Self::MirRight,
            '\\' => Self::MirLeft,
            '|' => Self::SplitV,
            '-' => Self::SplitH,
            _ => panic!("unknown ch: {ch}"),
        }
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
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
        assert_eq!(map.tiles.len(), 10 * 10);
        let in1 = include_str!("in1.txt");
        let map = Map::parse(in1);
        assert_eq!(map.cols, 110);
        assert_eq!(map.cols, 110);
        assert_eq!(map.tiles.len(), 110 * 110);
    }
}
