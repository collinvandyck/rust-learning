#![allow(unused, dead_code)]

use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("p1ex1 = {}", energized(ex1));
    println!("p1in1 = {}", energized(in1));
    println!("p2ex1 = {}", max_energized(ex1));
    println!("p2in1 = {}", max_energized(in1));
}

fn max_energized(input: &str) -> usize {
    let map = Map::parse(input);
    let top = (0..map.cols).map(|x| PointDir::new(Point::new(x, 0), Dir::Down));
    let btm = (0..map.cols).map(|x| PointDir::new(Point::new(x, map.rows - 1), Dir::Up));
    let lft = (0..map.rows).map(|y| PointDir::new(Point::new(0, y), Dir::Right));
    let rht = (0..map.rows).map(|y| PointDir::new(Point::new(map.cols - 1, y), Dir::Left));
    let all = top.chain(btm).chain(lft).chain(rht);
    all.par_bridge()
        .into_par_iter()
        .map(|pd| energized_map(&map, pd))
        .collect::<Vec<_>>()
        .into_iter()
        .max()
        .expect("no results")
}

fn energized(input: &str) -> usize {
    let map = Map::parse(input);
    let pd = PointDir::new(Point::new(0, 0), Dir::Right);
    energized_map(&map, pd)
}

fn energized_map(map: &Map, pd: PointDir) -> usize {
    let mut pds: HashSet<PointDir> = HashSet::default();
    let start = Beam::new(pd, map);
    let mut beams = vec![start];
    let mut energized: HashSet<Point> = HashSet::default();
    let mut paths = vec![];
    loop {
        if beams.is_empty() {
            break;
        }
        for idx in 0..beams.len() {
            if let Some(beam) = beams.get_mut(idx) {
                match beam.step(&map, &pds) {
                    BeamStep::Continue => {
                        pds.insert(beam.pd);
                    }
                    BeamStep::Split(new) => {
                        pds.insert(beam.pd);
                        pds.insert(new.pd);
                        beams.push(new);
                    }
                }
            }
        }
        beams.retain(|beam| {
            if beam.done {
                paths.push(beam.clone());
                energized.extend(beam.visited.0.iter().map(|pd| pd.pt));
            }
            !beam.done
        });
    }
    energized.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    tiles: Vec<TileXY>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Beam {
    path: Map,
    done: bool,
    visited: Visited,
    pd: PointDir,
}

#[derive(Debug, Clone, PartialEq, Eq, strum_macros::EnumIs)]
enum BeamStep {
    Continue,
    Split(Beam),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileXY {
    tile: Tile,
    pt: Point,
    ch: Option<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PointDir {
    pt: Point,
    dir: Dir,
}

impl Display for PointDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.dir, self.pt)
    }
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
    Visited,  // #
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIs, Hash)]
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
    fn start(&self) -> Beam {
        Beam::new(PointDir::new(Point::new(0, 0), Dir::Right), self)
    }
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
                        TileXY {
                            tile,
                            pt: point,
                            ch: None,
                        }
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
        self.idx(pt).and_then(|i| self.tiles.get(i)).copied()
    }
    fn idx(&self, pt: Point) -> Option<usize> {
        if pt.y < self.rows && pt.x < self.cols {
            Some(pt.y * self.cols + pt.x)
        } else {
            None
        }
    }
    fn must_idx(&self, pt: Point) -> usize {
        self.idx(pt).unwrap_or_else(|| panic!("bad pt: {pt:?}"))
    }
    fn set_tile(&mut self, pt: Point, tile: Tile) {
        let idx = self.must_idx(pt);
        if let Some(txy) = self.tiles.get_mut(idx) {
            txy.tile = tile;
        }
    }
    fn set_ch(&mut self, pt: Point, ch: char) {
        let idx = self.must_idx(pt);
        if let Some(txy) = self.tiles.get_mut(idx) {
            txy.ch = Some(ch);
        }
    }
    fn tidy(&mut self) {
        self.tiles.iter_mut().for_each(|txy| {
            if !txy.tile.is_visited() {
                txy.tile = Tile::Space;
            }
        })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tiles
            .chunks(self.cols)
            .map(|row| row.iter().map(|t| t.ch()).collect::<String>())
            .join("\n");
        writeln!(f, "{s}")
    }
}

impl Beam {
    fn new(pd: PointDir, map: &Map) -> Self {
        let mut visited = Visited::new();
        let done = false;
        let path = map.clone();
        let mut beam = Self {
            visited,
            pd,
            done,
            path,
        };
        beam.step(map, &HashSet::default());
        beam
    }
    fn step(&mut self, map: &Map, pds: &HashSet<PointDir>) -> BeamStep {
        let next = if self.visited.0.is_empty() {
            Some(self.pd.pt)
        } else {
            self.next_pt()
        };
        let next: TileXY = match next.and_then(|pt| map.get(pt)) {
            Some(next) => next,
            None => {
                self.done = true;
                return BeamStep::Continue;
            }
        };
        match next.tile {
            Tile::Space => {
                self.move_to(PointDir::new(next.pt, self.pd.dir), pds);
                return BeamStep::Continue;
            }
            Tile::SplitV => {
                if self.pd.dir.combo().is_up_down() {
                    self.move_to(PointDir::new(next.pt, self.pd.dir), pds);
                    return BeamStep::Continue;
                }
                let mut splt = self.fork(map);
                self.move_to(PointDir::new(next.pt, Dir::Up), pds);
                splt.move_to(PointDir::new(next.pt, Dir::Down), pds);
                return BeamStep::Split(splt);
            }
            Tile::SplitH => {
                if self.pd.dir.combo().is_left_right() {
                    self.move_to(PointDir::new(next.pt, self.pd.dir), pds);
                    return BeamStep::Continue;
                }
                let mut splt = self.fork(map);
                self.move_to(PointDir::new(next.pt, Dir::Left), pds);
                splt.move_to(PointDir::new(next.pt, Dir::Right), pds);
                return BeamStep::Split(splt);
            }
            Tile::MirRight => {
                let dir = match self.pd.dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                };
                self.move_to(PointDir::new(next.pt, dir), pds);
                return BeamStep::Continue;
            }
            Tile::MirLeft => {
                let dir = match self.pd.dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
                self.move_to(PointDir::new(next.pt, dir), pds);
                return BeamStep::Continue;
            }
            Tile::Visited => panic!("visited tile on map"),
        }
    }
    fn fork(&self, map: &Map) -> Self {
        let mut cloned = self.clone();
        cloned
    }
    fn next_pt(&self) -> Option<Point> {
        self.pd.pt.next(self.pd.dir)
    }
    fn move_to(&mut self, pd: PointDir, pds: &HashSet<PointDir>) {
        if pds.contains(&pd) {
            self.done = true;
            return;
        }
        if !self.visited.add(pd) {
            self.done = true;
        } else {
            self.pd = pd;
            self.path.set_ch(pd.pt, pd.dir.ch());
        }
    }
}

impl Display for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pd)
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
    fn ch(&self) -> char {
        match self {
            Tile::Space => '.',
            Tile::MirRight => '/',
            Tile::MirLeft => '\\',
            Tile::SplitV => '|',
            Tile::SplitH => '-',
            Tile::Visited => '#',
        }
    }
}

impl TileXY {
    fn ch(&self) -> char {
        if let Some(ch) = self.ch {
            ch
        } else {
            self.tile.ch()
        }
    }
}

impl Visited {
    fn new() -> Self {
        Self(vec![])
    }
    // returns true if the point did not already exist
    fn add(&mut self, pt: PointDir) -> bool {
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Dir {
    fn combo(&self) -> ComboDir {
        match self {
            Dir::Up | Dir::Down => ComboDir::UpDown,
            Dir::Left | Dir::Right => ComboDir::LeftRight,
        }
    }
    fn ch(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
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
