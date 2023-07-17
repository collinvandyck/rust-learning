use std::{collections::HashSet, fmt::Display, slice::Iter, sync::Arc};

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
}

impl<'a> Solver<'a> {
    fn new(game: Arc<&'a Game>) -> Self {
        let mut visited = HashSet::default();
        let path = vec![game.start];
        visited.insert(game.start);
        Self {
            game,
            visited,
            path,
        }
    }
    fn solve(&mut self) {
        // row,col is where we currently are
        let cur: Point = self.path.last().unwrap().clone();
        let map = &self.game.map;
        Self::directions()
            .map(|d| cur.next(d, map))
            .flatten()
            .for_each(|next| {
                dbg!(next);
            });
    }
    fn directions() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];
        DIRECTIONS.iter()
    }
}

impl Game {
    pub fn solve(&self) {
        let mut s = Solver::new(Arc::new(self));
        s.solve();
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
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::with_capacity(self.map.size() + self.map.rows());
        self.map.row_iter().enumerate().for_each(|(row_idx, row)| {
            buf.push_str(
                row.iter()
                    .enumerate()
                    .map(|(col_idx, t)| {
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
        write!(f, "{buf}")
    }
}
