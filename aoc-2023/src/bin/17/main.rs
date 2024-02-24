#![allow(dead_code, unused)]
use itertools::Itertools;
use std::fmt::Display;

fn main() {
    let ex = include_str!("ex1.txt");
    let map = Map::parse(ex);
    println!("ex1={}", map.heat_loss());
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tile {
    ch: char,
    val: u32,
    pt: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn dir(&self, dir: Dir) -> Option<Self> {
        match dir {
            Dir::Up => self
                .row
                .checked_sub(1)
                .map(|row| Self { row, col: self.col }),
            Dir::Down => Some(Self {
                row: self.row + 1,
                col: self.col,
            }),
            Dir::Left => self
                .col
                .checked_sub(1)
                .map(|col| Self { row: self.row, col }),
            Dir::Right => Some(Self {
                row: self.row,
                col: self.col + 1,
            }),
        }
    }
}

impl Tile {
    fn from(ch: char, row: usize, col: usize) -> Self {
        Self {
            ch,
            pt: Point { row, col },
            val: ch.to_digit(10).unwrap(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch)
    }
}

impl Map {
    fn heat_loss(&self) -> u32 {
        let start = self.get(0, 0).unwrap();
        let goal = self.get(self.rows() - 1, self.cols() - 1).unwrap();
        for (dir, tile) in self.neighbors(&start) {
            println!("Dir: {dir:?} tile: {tile}");
        }
        0
    }
    fn parse(s: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = s
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| Tile::from(ch, row, col))
                    .collect()
            })
            .collect();
        assert!(tiles.iter().map(|v| v.len()).all_equal());
        Self { tiles }
    }
    fn neighbors<'a>(&'a self, tile: &'a Tile) -> impl Iterator<Item = (Dir, Tile)> + 'a {
        NeighborIter::new(self, tile)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct NeighborIter<'a> {
    map: &'a Map,
    tile: &'a Tile,
    dirs: [Dir; 4],
    pos: usize,
}

impl<'a> Iterator for NeighborIter<'a> {
    type Item = (Dir, Tile);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= self.dirs.len() {
                return None;
            }
            let dir = self.dirs[self.pos];
            self.pos += 1;
            match self
                .tile
                .pt
                .dir(dir)
                .and_then(|pt| self.map.get(pt.row, pt.col).map(|tile| (dir, tile)))
            {
                Some(next) => return Some(next),
                None => continue,
            }
        }
    }
}

impl<'a> NeighborIter<'a> {
    fn new(map: &'a Map, tile: &'a Tile) -> Self {
        Self {
            map,
            tile,
            dirs: [Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            pos: 0,
        }
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
