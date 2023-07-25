use crate::prelude::*;

// the floor is at level y = 0. positions above
// the board are at y > 0.
pub struct Board {
    width: usize, // always 7
    entities: Vec<Entity>,
    shapes: Shapes,
    gusts: Gusts,
}

type Shapes = Box<dyn Iterator<Item = Shape>>;
type Gusts = Box<dyn Iterator<Item = Gust>>;

impl Board {
    pub fn new(shapes: Shapes, gusts: Gusts) -> Self {
        let width = 7;
        let entities = vec![];
        Self {
            width,
            entities,
            shapes,
            gusts,
        }
    }
    pub fn run(&mut self) {
        let shape = self.shapes.next();
        println!("Running... {shape:?}");
    }
}

// The point is the coordinate of the upper left part of the shape.
struct Entity(Shape, Point);

enum Tile {
    Empty,
    Rock,
}

struct Point {
    x: usize,
    y: usize,
}
