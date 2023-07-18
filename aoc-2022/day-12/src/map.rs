use crate::*;

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    pub fn new(tiles: Vec<Tile>, width: usize) -> Self {
        Self { tiles, width }
    }
    pub fn tile<'a>(&'a self, point: &Point) -> Option<&'a Tile> {
        let idx = self.idx(point);
        self.tiles.get(idx)
    }
    fn idx(&self, point: &Point) -> usize {
        point.row * self.cols() + point.col
    }
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }
    pub fn size(&self) -> usize {
        self.cols() * self.rows()
    }
    pub fn cols(&self) -> usize {
        self.width
    }
    pub fn rows(&self) -> usize {
        self.tiles.len() / self.width
    }
    pub fn row_iter(&self) -> impl Iterator<Item = &[Tile]> {
        self.tiles.chunks(self.cols())
    }
}
