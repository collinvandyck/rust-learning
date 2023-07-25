// seven units wide
pub struct Board {
    width: usize, // always 7
    rows: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new() -> Self {
        let width = 7;
        let rows = vec![];
        Self { width, rows }
    }
}

enum Tile {
    Empty,
    Rock,
}

struct Point {
    x: usize,
    y: usize,
}
