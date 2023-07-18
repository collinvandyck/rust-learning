use std::{
    collections::HashSet,
    fmt::Display,
    slice::Iter,
    sync::{Arc, Mutex},
};

use crate::prelude::*;

#[derive(Clone)]
pub struct Game {
    map: Map,
    start: Point,
    finish: Point,
}

struct Solver<'a> {
    game: Arc<&'a Game>,
    visited: HashSet<Point>,
    path: Vec<Point>,
    solution: Arc<Mutex<Option<Vec<Point>>>>,
}

impl<'a> Solver<'a> {
    fn new(game: Arc<&'a Game>) -> Self {
        let solution = Arc::default();
        let mut visited = HashSet::default();
        let path = vec![game.start];
        visited.insert(game.start);
        Self {
            game,
            visited,
            path,
            solution,
        }
    }
    fn solve(&mut self) -> bool {
        // row,col is where we currently are
        let cur: Point = self.path.last().unwrap().clone();
        println!("Solve cur:{cur:?}");

        // check to see if we're done.
        if cur == self.game.finish {
            println!("Solved:\n{self}");
            self.path.iter().for_each(|p| {
                println!("{p:?}");
            });
            return true;
        }

        let map = &self.game.map;
        for direction in Self::directions() {
            if let Some(next) = cur.next(direction, map) {
                if self.visited.contains(&next) {
                    continue;
                }
                let cur_tile = self.game.map.tile(&cur).unwrap();
                let nex_tile = self.game.map.tile(&next).unwrap();
                if !cur_tile.can_move_to(&nex_tile) {
                    continue;
                }
                self.visited.insert(next.clone());
                self.path.push(next.clone());
                if self.solve() {
                    return true;
                } else {
                    self.path.pop();
                }
            }
        }
        false
    }
    fn directions() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Down, Up, Right, Left];
        DIRECTIONS.iter()
    }
}

impl<'a> Display for Solver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.game.render(Some(&self.path));
        write!(f, "{s}")
    }
}

impl Game {
    pub fn solve(&self) {
        let mut s = Solver::new(Arc::new(self));
        if s.solve() {
            println!("Solved!");
        } else {
            println!("Could not solve.");
        }
    }
}

impl Game {
    fn new(map: Map, start: Point, finish: Point) -> Self {
        Self { map, start, finish }
    }
    pub fn from_iter(iter: impl Iterator<Item = String>) -> Self {
        let mut tiles = vec![];
        let mut width = 0;
        let mut start = Point::new(0, 0);
        let mut finish = Point::new(0, 0);
        for (row, line) in iter.enumerate() {
            width = line.len();
            let mut row: Vec<Tile> = line
                .chars()
                .enumerate()
                .map(|(col, c)| {
                    let c = match c {
                        'S' => {
                            start = Point::new(row, col);
                            'a'
                        }
                        'E' => {
                            finish = Point::new(row, col);
                            'z'
                        }
                        c => c,
                    };
                    Tile(c)
                })
                .collect();
            tiles.append(&mut row);
        }
        let map = Map::new(tiles, width);
        Self::new(map, start, finish)
    }
    fn render(&self, paths: Option<&Vec<Point>>) -> String {
        let mut buf = String::with_capacity(self.map.size() + self.map.rows());
        self.map.row_iter().enumerate().for_each(|(row_idx, row)| {
            buf.push_str(
                row.iter()
                    .enumerate()
                    .map(|(col_idx, t)| {
                        if let Some(paths) = paths {
                            if paths
                                .iter()
                                .find(|point| point.matches(row_idx, col_idx))
                                .is_some()
                            {
                                return '#';
                            }
                        }
                        if self.start == Point::new(row_idx, col_idx) {
                            'S'
                        } else if self.finish == Point::new(row_idx, col_idx) {
                            'E'
                        } else {
                            t.0
                        }
                    })
                    .collect::<String>()
                    .as_str(),
            );
            if row_idx < self.map.rows() - 1 {
                buf.push('\n');
            }
        });
        buf
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render(None))
    }
}
