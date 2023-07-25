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
    fn add_shape(&mut self, shape: Shape) {
        let mut points = Points(shape.starting_coords());
        self.adjust_points_for_insert(&mut points);
        let entity = Entity { shape, points };
        self.entities.push(entity);
    }
    /// Each rock appears so that its left edge is two units away from
    /// the left wall and its bottom edge is three units above the highest
    /// rock in the room (or the floor, if there isn't one).
    fn adjust_points_for_insert(&mut self, points: &mut Points) {}
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
    points: Points,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Points(Vec<Point>);
