#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("example.txt");
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize); // row,col

#[derive(Debug)]
struct Path(Vec<Point>);

struct Visited(HashSet<Point>);

struct Solver<'a> {
    map: &'a Map,
    path: Path,
    visited: Visited,
    cur: Option<Point>,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Self {
        let path = Path(vec![]);
        let visited = Visited(HashSet::new());
        Self {
            map,
            path,
            visited,
            cur: None,
        }
    }
    // solve attempts to find the shortest path from the start to the end.
    fn solve(&'a mut self) -> &'a Path {
        &self.path
    }
}

struct Map {
    tiles: Vec<Vec<char>>,
    start: Point,
    finish: Point,
    rows: usize,
    cols: usize,
}

impl Map {
    fn solve(&self) {
        println!("Solve:\n{self}");
        let mut solver = Solver::new(self);
        let path = solver.solve();
        println!("Solution: {path:?}");
    }
    fn render(&self) -> String {
        self.tiles
            .iter()
            .enumerate()
            .map(|(row, rows)| {
                rows.iter()
                    .enumerate()
                    .map(|(col, char)| {
                        let point = Point(row, col);
                        if self.start == point {
                            &'S'
                        } else if self.finish == point {
                            &'E'
                        } else {
                            char
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn new(tiles: Vec<Vec<char>>, start: Point, finish: Point) -> Self {
        let rows = tiles.len();
        let cols = tiles.get(0).map_or(0, |r| r.len());
        Self {
            tiles,
            start,
            finish,
            rows,
            cols,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

fn read_map(iter: impl Iterator<Item = String>) -> Map {
    let mut tiles = vec![];
    let mut start = Point(0, 0);
    let mut finish = Point(0, 0);
    for (row, line) in iter.enumerate() {
        let row: Vec<char> = line
            .chars()
            .enumerate()
            .map(|(col, c)| match c {
                'S' => {
                    start = Point(row, col);
                    'a'
                }
                'E' => {
                    finish = Point(row, col);
                    'z'
                }
                c => c,
            })
            .collect();
        tiles.push(row);
    }
    Map::new(tiles, start, finish)
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let lines = read.lines().flatten();
    let mut map = read_map(lines);
    map.solve();
}
