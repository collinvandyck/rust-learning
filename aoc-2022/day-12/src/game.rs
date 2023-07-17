use std::fmt::Display;

use crate::prelude::*;

pub struct Game {
    map: Map,
    start: Point,
    finish: Point,
}

impl Game {
    pub fn solve(&self) {}

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
        Self { map, start, finish }
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
            buf.push('\n');
        });
        write!(f, "{buf}")
    }
}
