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

impl From<char> for Gust {
    fn from(value: char) -> Self {
        match value {
            '<' => Gust::Left,
            '>' => Gust::Right,
            _ => panic!("Invalid char: {value}"),
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
