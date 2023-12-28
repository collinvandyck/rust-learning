#![allow(dead_code, unused)]

use std::collections::HashSet;

use itertools::Itertools;

fn main() {}

fn energized(input: &str) -> usize {
    let map = Map::parse(input);
    let mut beams = vec![];
    let mut energized: HashSet<Point> = HashSet::default();
    beams.push(Beam::new(Point::new(0, 0), Dir::Right));
    while !beams.is_empty() {
        for idx in 0..beams.len() {
            let mut remove = false;
            if let Some(beam) = beams.get_mut(idx) {
                match beam.step(&map) {
                    BeamStep::Continue => {}
                    BeamStep::Finished => {
                        energized.extend(beam.visited.0.iter().map(|pd| pd.pt).collect_vec());
                        remove = true;
                    }
                    BeamStep::Split(new) => {
                        beams.push(new);
                    }
                }
            }
            if remove {
                beams.remove(idx);
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
    visited: Visited,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PointDir {
    pt: Point,
    dir: Dir,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Visited(Vec<PointDir>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs)]
enum ComboDir {
    UpDown,
    LeftRight,
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
    fn get(&self, pt: Point) -> Option<TileXY> {
        let idx = self.idx(pt);
        self.tiles.get(idx).copied()
    }
    fn idx(&self, pt: Point) -> usize {
        pt.y * self.cols + pt.x
    }
}

impl Beam {
    fn new(pt: Point, dir: Dir) -> Self {
        let mut visited = Visited::new();
        visited.add(pt, dir);
        Self { visited, pt, dir }
    }
    fn step(&mut self, map: &Map) -> BeamStep {
        let next: TileXY = match self.next_pt().and_then(|pt| map.get(pt)) {
            Some(next) => next,
            None => return BeamStep::Finished,
        };
        match next.tile {
            Tile::Space => {
                self.move_to(next.pt, self.dir);
                return BeamStep::Continue;
            }
            Tile::SplitV if self.dir.combo().is_up_down() => {
                self.move_to(next.pt, self.dir);
                return BeamStep::Continue;
            }
            Tile::SplitH if self.dir.combo().is_left_right() => {
                self.move_to(next.pt, self.dir);
                return BeamStep::Continue;
            }
            Tile::SplitV => {
                let mut splt = self.clone();
                self.move_to(next.pt, Dir::Up);
                splt.move_to(next.pt, Dir::Down);
                return BeamStep::Split(splt);
            }
            Tile::SplitH => {
                let mut splt = self.clone();
                self.move_to(next.pt, Dir::Left);
                splt.move_to(next.pt, Dir::Right);
                return BeamStep::Split(splt);
            }
            Tile::MirRight => {
                let dir = match self.dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                };
                self.move_to(next.pt, dir);
                return BeamStep::Continue;
            }
            Tile::MirLeft => {
                let dir = match self.dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
                self.move_to(next.pt, dir);
                return BeamStep::Continue;
            }
        }
    }
    fn next_pt(&self) -> Option<Point> {
        self.pt.next(self.dir)
    }
    fn move_to(&mut self, pt: Point, dir: Dir) {
        self.visited.add(pt, dir);
        self.pt = pt;
        self.dir = dir;
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

impl Visited {
    fn new() -> Self {
        Self(vec![])
    }
    // returns true if the point did not already exist
    fn add(&mut self, pt: Point, dir: Dir) -> bool {
        let pt = PointDir::new(pt, dir);
        if self.contains(&pt) {
            false
        } else {
            self.0.push(pt);
            true
        }
    }
    fn contains(&self, pt: &PointDir) -> bool {
        self.0.contains(pt)
    }
}

impl PointDir {
    fn new(pt: Point, dir: Dir) -> Self {
        Self { pt, dir }
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

impl Dir {
    fn combo(&self) -> ComboDir {
        match self {
            Dir::Up | Dir::Down => ComboDir::UpDown,
            Dir::Left | Dir::Right => ComboDir::LeftRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        let ex1 = include_str!("ex1.txt");
        let egs = energized(ex1);
        assert_eq!(egs, 46);
    }

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
