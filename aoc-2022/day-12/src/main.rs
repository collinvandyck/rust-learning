#![allow(dead_code, unused)]
#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    slice::SliceIndex,
};

fn main() {
    run("example-small.txt");
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(usize, usize); // row,col

impl Point {
    fn adjust(&self, rows: i32, cols: i32) -> Option<Point> {
        if rows < 0 && self.0 == 0 {
            None
        } else if cols < 0 && self.1 == 0 {
            None
        } else {
            let new_rows = (self.0 as i32 + rows) as usize;
            let new_cols = (self.1 as i32 + cols) as usize;
            Some(Self(new_rows, new_cols))
        }
    }
}

struct Solver<'a> {
    map: &'a Map,
}

impl<'a> Solver<'a> {
    fn new(map: &'a Map) -> Self {
        Self { map }
    }
    // solve attempts to find the shortest path from the start to the end.
    fn solve(&mut self) -> Option<Vec<Point>> {
        let path = vec![self.map.start];
        let visited = HashSet::from([self.map.start]);
        self.do_solve(path, visited)
    }
    fn do_solve(
        &mut self,
        mut path: Vec<Point>,
        mut visited: HashSet<Point>,
    ) -> Option<Vec<Point>> {
        let current = path.last().unwrap();
        if current == &self.map.finish {
            return Some(path);
        }
        // we're not done yet. try to solve in possibly four directions.
        let nexts = self.map.nexts(current);
        for next in nexts {}
        dbg!(nexts);
        None
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
    fn nexts(&self, cur: &Point) -> [Option<Point>; 4] {
        [
            cur.adjust(-1, 0),
            cur.adjust(1, 0),
            cur.adjust(0, -1),
            cur.adjust(0, 1),
        ]
        .map(|p| match p {
            Some(p) => {
                if p.0 > self.rows - 1 || p.1 > self.cols - 1 {
                    // out of bounds
                    None
                } else {
                    if self.can_move(cur, &p) {
                        Some(p)
                    } else {
                        // can't move
                        None
                    }
                }
            }
            // out of bounds
            _ => None,
        })
    }
    fn can_move(&self, from: &Point, to: &Point) -> bool {
        let (from, to) = dbg!(from, to);
        let from = dbg!(self.get_char(&from)) as u8;
        let to = dbg!(self.get_char(&to)) as u8;
        dbg!(to - from) <= 1
    }
    fn get_char(&self, p: &Point) -> char {
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
    let map = read_map(lines);
    map.solve();
}
