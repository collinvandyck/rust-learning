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

impl Path {
    fn last(&self) -> Option<&Point> {
        self.0.last()
    }
    fn add(&mut self, point: Point) {
        self.0.push(point);
    }
}

struct Visited(HashSet<Point>);

impl Visited {
    fn add(&mut self, point: Point) {
        if !self.0.insert(point) {
            panic!("visited already contains {point:?}");
        }
    }
    fn contains(&self, point: &Point) -> bool {
        self.0.contains(point)
    }
}

struct Solver<'a> {
    map: &'a Map,
    path: Path,
    visited: Visited,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Self {
        let path = Path(vec![]);
        let visited = Visited(HashSet::new());
        Self { map, path, visited }
    }
    // solve attempts to find the shortest path from the start to the end.
    fn solve(&'a mut self) -> &'a Path {
        for next in self.next_points() {
            dbg!(next);
            self.path.add(next);
            self.visited.add(next);
        }
        &self.path
    }
    fn next_points(&self) -> impl Iterator<Item = Point> {
        let mut v = Vec::with_capacity(4);
        match self.path.last() {
            Some(cur @ &Point(row, col)) => {
                if row > 0 {
                    let point = Point(row - 1, col);
                    if self.can_move(cur, &point) {
                        v.push(point);
                    }
                }
                if col > 0 {
                    let point = Point(row, col - 1);
                    if self.can_move(cur, &point) {
                        v.push(point);
                    }
                }
                if row < self.map.rows - 1 {
                    let point = Point(row + 1, col);
                    if self.can_move(cur, &point) {
                        v.push(point);
                    }
                }
                if col < self.map.cols - 1 {
                    let point = Point(row, col + 1);
                    if self.can_move(cur, &point) {
                        v.push(point);
                    }
                }
            }
            None => v.push(self.map.start),
        }
        v.into_iter()
    }
    fn can_move(&self, from: &Point, to: &Point) -> bool {
        if self.visited.contains(to) {
            return false;
        }
        let distance = self.map.distance(from, to);
        distance <= 1
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
    fn distance(&self, from: &Point, to: &Point) -> i32 {
        let from = self.get(from) as i32;
        let to = self.get(to) as i32;
        to - from
    }
    fn get(&self, p: &Point) -> char {
        *self.tiles.get(p.0).unwrap().get(p.1).unwrap()
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
