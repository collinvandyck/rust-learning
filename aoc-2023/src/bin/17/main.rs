#![allow(dead_code, unused)]

use itertools::Itertools;

fn main() {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    tiles: Vec<Tile>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .map(|ch| ch.to_string().parse::<usize>().unwrap())
                    .enumerate()
                    .map(move |(x, cost)| Tile::new(Point::new(x, y), cost))
            })
            .collect_vec();
        Self { tiles }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    pt: Point,
    cost: usize,
}

impl Tile {
    fn new(pt: Point, cost: usize) -> Self {
        Self { pt, cost }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIs)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
