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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gust {
    Left,
    Right,
}

impl Into<Gust> for char {
    fn into(self) -> Gust {
        match self {
            '<' => Gust::Left,
            '>' => Gust::Right,
            _ => panic!("Invalid gust: {self}"),
        }
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
