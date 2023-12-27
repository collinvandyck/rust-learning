#![allow(dead_code, unused)]

use std::{fmt::Display, time::Instant};

use itertools::Itertools;
fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");

    println!("p1ex1 = {}", tilt_load(ex1, Dir::North));
    println!("p1in1 = {}", tilt_load(in1, Dir::North));

    let start = Instant::now();
    cycle(ex1, 1000000);
}

fn cycle(input: &str, amt: usize) {
    let mut map = Map::parse(input);
    let start = Instant::now();
    for _ in 0..amt {
        map.tilt(Dir::North);
    }
    let dur = start.elapsed();
    let per_sec = (amt as f64) / dur.as_secs_f64();
    println!("cycle dur: {dur:?} per/sec: {per_sec:.2}");
}

fn tilt_load(input: &str, dir: Dir) -> usize {
    let mut map = Map::parse(input);
    map.tilt(dir);
    map.load()
}

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

    fn load(&self) -> usize {
        let rows = self.rows;
        let cols = self.cols;
        (0..cols)
            .map(|x| {
                (0..rows)
                    .map(move |y| (x, y))
                    .map(|(x, y)| (y, self.get_xy(x, y)))
                    .filter(|(_, t)| t == &Tile::Round)
                    .map(|(mut y, _t)| rows - y)
                    .sum::<usize>()
            })
            .sum()
    }

    fn tilt(&mut self, dir: Dir) {
        let rows = self.rows;
        let cols = self.cols;
        match dir {
            Dir::North => (1..rows)
                .flat_map(|start| (1..=start).rev())
                .flat_map(|y| (0..cols).map(move |x| (x, y, y - 1)))
                .for_each(|v @ (x, ys, yd)| self.drop_rock((x, ys), (x, yd))),
            _ => panic!("unsupported dir"),
        }
    }

    fn drop_rock(&mut self, src: (usize, usize), dst: (usize, usize)) {
        let st = self.get_xy(src.0, src.1);
        let dt = self.get_xy(dst.0, dst.1);
        if st == Tile::Round && dt == Tile::Space {
            self.set_xy(src.0, src.1, Tile::Space);
            self.set_xy(dst.0, dst.1, Tile::Round);
        }
    }

    fn set_xy(&mut self, x: usize, y: usize, tile: Tile) {
        let dt = self
            .tiles
            .get_mut(self.cols * y + x)
            .expect("no tile found");
        *dt = tile
    }

    fn get_xy(&self, x: usize, y: usize) -> Tile {
        self.tiles
            .get(self.cols * y + x)
            .expect("no tile found")
            .clone()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "tmp"]
    #[test]
    fn test_cycle() {
        let ex1 = include_str!("ex1.txt");
        let mut map = Map::parse(ex1);
        for _ in 0..1000 {
            map.tilt(Dir::North);
        }
    }

    #[test]
    fn test_pt1() {
        let ex1 = include_str!("ex1.txt");
        let mut map = Map::parse(ex1);
        map.tilt(Dir::North);
        println!("{map}");
        assert_eq!(map.load(), 136);
    }

    #[test]
    fn test_drop() {
        let ex1 = include_str!("ex1.txt");
        let ex1n = include_str!("ex1_north.txt");
        let mut map = Map::parse(ex1);
        map.tilt(Dir::North);
        assert_eq!(map.to_string(), ex1n);
    }

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
