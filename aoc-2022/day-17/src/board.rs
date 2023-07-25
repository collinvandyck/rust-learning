use crate::prelude::*;

// the floor is at level y = 0. positions above
// the board are at y > 0.
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
        let shape = self.shapes.next();
        println!("Running... {shape:?}");
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
