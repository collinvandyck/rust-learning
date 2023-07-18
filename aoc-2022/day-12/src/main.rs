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
    fn push(&mut self, point: Point) {
        self.0.push(point);
    }
    fn pop(&mut self) {
        self.0.pop();
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn append(&mut self, mut other: Path) {
        self.0.append(&mut other.0);
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
    visited: Visited,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Self {
        let visited = Visited(HashSet::new());
        Self { map, visited }
    }
    // solve attempts to find the shortest path from the start to the end.
    fn solve(&'a mut self) -> Option<Path> {
        self.do_solve(self.map.start)
    }
    fn do_solve(&mut self, mut cur: Point) -> Option<Path> {
        cur = dbg!(cur);
        let mut path = Path(vec![cur]);

        // check to see if solved
        if self.is_finished(cur) {
            return Some(path);
        }
        // mark the cur point as being visited
        self.visited.add(cur);

        // recurse into each direction
        let mut res: Option<Path> = None;
        for next in self.next_points(&cur) {
            if let Some(next_path) = self.do_solve(next) {
                res = match res {
                    None => Some(next_path),
                    Some(r) if r.len() < next_path.len() => Some(r),
                    _ => Some(next_path),
                }
            }
        }
        // if there was a result, append it to our path
        res.map(|r| {
            path.append(r);
            path
        })
    }
    fn is_finished(&self, point: Point) -> bool {
        point == self.map.finish
    }
    fn next_points(&self, cur: &Point) -> impl Iterator<Item = Point> {
        let mut v = Vec::with_capacity(4);
        let Point(row, col) = cur.clone();
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
