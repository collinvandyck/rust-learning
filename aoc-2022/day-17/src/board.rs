use std::{
    fmt::{Display, Write},
    ops::{AddAssign, Deref, DerefMut},
};

use crate::prelude::*;

impl Board {
    pub fn new() -> Self {
        let width = 7;
        let entities = vec![];
        Self { width, entities }
    }

    pub fn run(&mut self, mut shapes: Shapes, mut gusts: Gusts) {
        println!("{self}");
        let shape = shapes.next().unwrap();
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
    fn adjust_points_for_insert(&mut self, points: &mut Points) {
        let adjust = Point(2, 3 + self.highest_rock_y());
        points.iter_mut().for_each(|p| *p += adjust);
    }

    /// returns the highest rock y position. The floor is represented at y=0.
    fn highest_rock_y(&self) -> i32 {
        self.entities
            .iter()
            .flat_map(|e| e.points.0.iter())
            .map(|p| p.1)
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board_highest_rock_y() {
        let mut b = Board::new();
        assert_eq!(b.highest_rock_y(), 0);

        // |..##...|
        // |..##...|
        // |.......|
        // |.......|
        // |.......|
        // +-------+
        let shape = Shape::Square;
        b.add_shape(shape);
        assert_eq!(b.highest_rock_y(), 5);

        let shape = Shape::Pipe;
        b.add_shape(shape);
        assert_eq!(b.highest_rock_y(), 3);
    }
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
}

type Shapes = Box<dyn Iterator<Item = Shape>>;
type Gusts = Box<dyn Iterator<Item = Gust>>;

struct Entity {
    shape: Shape,
    points: Points,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Points(Vec<Point>);

impl Deref for Points {
    type Target = Vec<Point>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Points {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
