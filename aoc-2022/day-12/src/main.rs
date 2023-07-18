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
    run("example.txt");
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
        //let current = dbg!(current);
        if current == &self.map.finish {
            return Some(path);
        }
        visited.insert(*current);
        let nexts = self.map.nexts(current);
        let mut suffix: Option<Vec<Point>> = None;
        for next in nexts.into_iter().flatten() {
            if visited.contains(&next) {
                continue;
            }
            let mut path = path.clone();
            path.push(next);
            let visited = visited.clone();
            let next_res = self.do_solve(path, visited);
            if let Some(next_res) = next_res {
                suffix = match suffix {
                    None => Some(next_res),
                    Some(existing) if next_res.len() < existing.len() => Some(next_res),
                    _ => suffix,
                }
            }
        }
        if let Some(mut suffix) = suffix {
            path.append(&mut suffix);
            Some(path)
        } else {
            None
        }
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
        println!("Solve:\n{self}\n");
        let mut solver = Solver::new(self);
        match solver.solve() {
            None => eprintln!("No solution."),
            Some(path) => {
                let rendered = self.render_path(path);
                println!("Solution:\n{rendered}");
            }
        }
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
        let from = self.get_char(from) as u8;
        let to = self.get_char(to) as u8;
        if to <= from {
            return true;
        }
        // to is > from. we must make sure the difference is only 1
        if to == from + 1 {
            return true;
        }
        false
    }
    fn get_char(&self, p: &Point) -> char {
        *self.tiles.get(p.0).unwrap().get(p.1).unwrap()
    }
    fn render_path(&self, path: Vec<Point>) -> String {
        let tiles = self.tiles.clone();
        tiles
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
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
