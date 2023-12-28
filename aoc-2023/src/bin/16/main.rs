#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {}

fn energized(input: &str) -> usize {
    let map = Map::parse(input);
    let mut beams = vec![];
    beams.push(Beam::new(Point::new(0, 0), Dir::Right));
    while !beams.is_empty() {
        for idx in 0..beams.len() {
            if let Some(beam) = beams.get_mut(idx) {
                match beam.step(&map) {
                    BeamStep::Continue => todo!(),
                    BeamStep::Finished => todo!(),
                    BeamStep::Split(_) => todo!(),
                }
            }
        }
    }
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    tiles: Vec<TileXY>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Beam {
    visited: Points,
    pt: Point,
    dir: Dir,
}

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::EnumIs)]
enum BeamStep {
    Continue,
    Finished,
    Split(Beam),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileXY {
    tile: Tile,
    pt: Point,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Points(Vec<Point>);

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
                        TileXY { tile, pt: point }
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

impl Beam {
    fn new(pt: Point, dir: Dir) -> Self {
        let mut visited = Points::new();
        visited.add(pt);
        Self { visited, pt, dir }
    }
    fn step(&mut self, map: &Map) -> BeamStep {
        todo!()
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

impl Points {
    fn new() -> Self {
        Self(vec![])
    }
    // returns true if the point did not already exist
    fn add(&mut self, pt: Point) -> bool {
        if self.contains(&pt) {
            false
        } else {
            self.0.push(pt);
            true
        }
    }
    fn contains(&self, pt: &Point) -> bool {
        self.0.contains(pt)
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn next(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Up => self.y.checked_sub(1).map(|y| (self.x, y)),
            Dir::Down => self.y.checked_add(1).map(|y| (self.x, y)),
            Dir::Left => self.x.checked_sub(1).map(|x| (x, self.y)),
            Dir::Right => self.x.checked_add(1).map(|x| (x, self.y)),
        }
        .map(|(x, y)| Point::new(x, y))
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
