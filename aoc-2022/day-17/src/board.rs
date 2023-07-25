// seven units wide
pub struct Chamber {
    width: usize, // always 7
    rows: Vec<Vec<Tile>>,
}

impl Chamber {
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

fn shapes() -> [Shape; 5] {
    [
        Shape::Slab,
        Shape::Cross,
        Shape::L,
        Shape::Pipe,
        Shape::Square,
    ]
}

#[derive(Clone, Copy)]
enum Shape {
    Slab,
    Cross,
    L,
    Pipe,
    Square,
}
