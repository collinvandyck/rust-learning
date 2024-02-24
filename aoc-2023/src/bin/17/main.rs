#![allow(dead_code, unused)]
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{binary_heap, hash_map::Entry, HashMap},
    fmt::Display,
    time::{Duration, Instant},
    u32,
};

fn main() {
    let ex1 = include_str!("ex1.txt");
    let in1 = include_str!("in1.txt");
    println!("ex1={}", part_1(ex1));
    println!("in1={}", part_1(in1));
}

struct Timed<T> {
    val: T,
    dur: Duration,
}

impl<T: Display> Display for Timed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:.03?}s)", self.val, self.dur.as_secs_f64())
    }
}

fn part_1(input: &str) -> Timed<u32> {
    let start = Instant::now();
    let map = Map::parse(input);
    Timed {
        val: map.heat_loss(),
        dur: start.elapsed(),
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Tile {
    ch: char,
    val: u32,
    pt: Point,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    tile: Tile,
    cost: u32,
    prev: [Option<Dir>; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StateKey {
    tile: Tile,
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
    fn key(&self) -> StateKey {
        StateKey {
            tile: self.tile,
            prev: self.prev,
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
        let mut full_cost: Option<u32> = None;
        let mut visited: HashMap<Tile, HashMap<StateKey, u32>> = HashMap::default();
        while let Some(state) = queue.pop() {
            let next = self
                .neighbors(&state.tile)
                .flat_map(|(dir, tile)| state.next(dir, tile));
            for mut next in next {
                let new_cost = state.cost + next.tile.val;
                if full_cost.map(|fc| fc < new_cost).unwrap_or_default() {
                    continue;
                }
                let key = next.key();
                let old_cost = visited
                    .entry(next.tile)
                    .or_insert_with(|| HashMap::from([(key, new_cost)]))
                    .entry(key)
                    .or_insert(new_cost);
                if &new_cost > old_cost {
                    continue;
                }
                next.cost = new_cost;
                if next.tile == goal {
                    full_cost.replace(next.cost);
                } else {
                    queue.push(next);
                }
            }
        }
        full_cost.unwrap_or_default()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
