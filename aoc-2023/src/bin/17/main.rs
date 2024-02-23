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

struct Crucible<'a> {
    map: &'a Map,
    pos: Point,
    start: Point,
    end: Point,
}

impl<'a> Crucible<'a> {
    fn new(map: &'a Map, start: Point, end: Point) -> Self {
        Self {
            map,
            pos: start,
            start,
            end,
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

impl Map {
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
    fn heat_loss(&self) -> u32 {
        todo!()
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
