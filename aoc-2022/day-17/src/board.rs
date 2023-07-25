use crate::prelude::*;

// seven units wide
pub struct Board {
    width: usize, // always 7
    rows: Vec<Vec<Tile>>,
    shapes: Shapes,
    gusts: Gusts,
}

type Shapes = Box<dyn Iterator<Item = Shape>>;
type Gusts = Box<dyn Iterator<Item = Gust>>;

impl Board {
    pub fn new(shapes: Shapes, gusts: Gusts) -> Self {
        let width = 7;
        let rows = vec![];
        Self {
            width,
            rows,
            shapes,
            gusts,
        }
    }
    pub fn run(&mut self) {
        println!("Running...");
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
