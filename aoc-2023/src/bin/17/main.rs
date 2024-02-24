#![allow(dead_code, unused)]
use itertools::Itertools;
use std::{cmp::Ordering, collections::binary_heap, fmt::Display};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    tile: Tile,
    cost: u32,
    prev: [Option<Dir>; 3],
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.tile, self.cost)
    }
}

impl State {
    fn new(tile: Tile, cost: u32) -> Self {
        Self {
            tile,
            cost,
            prev: [None, None, None], // most recent first
        }
    }
    // returns the next state with the move to the specified tile. if the move is not allowed none
    // will be returned
    fn next(&self, dir: Dir, tile: Tile) -> Option<Self> {
        if self
            .prev
            .iter()
            .all(|d| d.map(|d| d == dir).unwrap_or_default())
        {
            // disallow more than three moves in the same dir
            return None;
        }
        if self.prev[0]
            .map(|d| d.opposite() == dir)
            .unwrap_or_default()
        {
            // disallow 180 degree turns
            return None;
        }
        // here we are allowed
        Some(Self {
            tile,
            cost: self.cost + tile.val,
            prev: [Some(dir), self.prev[0], self.prev[1]],
        })
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn heat_loss(&self) -> u32 {
        let start = self.get(0, 0).unwrap();
        let goal = self.get(self.rows() - 1, self.cols() - 1).unwrap();
        let mut queue = {
            let start = State::new(start, 0);
            let mut queue = binary_heap::BinaryHeap::new();
            queue.push(start);
            queue
        };
        while let Some(state) = queue.pop() {
            println!("State: {state}");
            for next in self
                .neighbors(&state.tile)
                .flat_map(|(dir, tile)| state.next(dir, tile))
            {
                //
            }
            for (dir, tile) in self.neighbors(&state.tile) {
                let next = state.next(dir, tile);
                println!("Dir: {dir:?} tile: {tile}");
            }
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

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
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
