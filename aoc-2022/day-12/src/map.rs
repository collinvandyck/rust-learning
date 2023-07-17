use std::fmt::{Display, Write};

use crate::*;

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    pub fn from_iter(mut iter: impl Iterator<Item = String>) -> Self {
        let mut tiles = vec![];
        let mut width = 0;
        while let Some(line) = iter.next() {
            width = line.len();
            let mut row: Vec<Tile> = line.chars().map(|c| Tile(c)).collect();
            tiles.append(&mut row);
        }
        Self { tiles, width }
    }
    pub fn solve(&self) {}

    fn size(&self) -> usize {
        self.cols() * self.rows()
    }
    fn cols(&self) -> usize {
        self.width
    }
    fn rows(&self) -> usize {
        self.tiles.len() / self.width
    }
    fn row_iter(&self) -> impl Iterator<Item = &[Tile]> {
        self.tiles.chunks(self.cols())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::with_capacity(self.size() + self.rows());
        self.row_iter().for_each(|row| {
            buf.push_str(row.iter().map(|t| t.0).collect::<String>().as_str());
            buf.push('\n');
        });
        write!(f, "{buf}")
    }
}
