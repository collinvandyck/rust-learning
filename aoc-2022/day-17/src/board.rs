use std::fmt::{Display, Write};

use crate::prelude::*;

impl Board {
    pub fn run(&mut self) {
        println!("{self}");
        let shape = self.shapes.next().unwrap();
        self.add_shape(shape);
    }
    fn render(&self) -> String {
        let mut buf = String::new();
        // TODO: print the shapes
        // print the base.
        let _ = write!(buf, "+{}+", "-".repeat(self.width)).unwrap();
        buf
    }
    fn add_shape(&mut self, shape: Shape) {}
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

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
}

struct Entity {
    shape: Shape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);
