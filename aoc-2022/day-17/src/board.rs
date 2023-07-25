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
        println!("{self}");
    }

    fn render(&self) -> String {
        let mut points = self.sorted_points().into_iter().peekable();
        let mut lines = vec![];
        for y in (1..=self.highest_rock_y()).rev() {
            let mut line = String::new();
            for x in 1..=7 {
                if points.peek().filter(|p| p.0 == x && p.1 == y).is_some() {
                    line.push('#');
                    points.next();
                } else {
                    line.push('.');
                }
            }
            lines.push(line);
        }
        lines.push(format!("+{}+", "-".repeat(self.width)));
        lines.join("\n")
    }

    fn sorted_points(&self) -> Vec<Point> {
        let mut points = self
            .entities
            .iter()
            .flat_map(|e| e.points.iter())
            .copied()
            .collect::<Vec<_>>();
        points.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        points
    }

    /// Each rock appears so that its left edge is two units away from
    /// the left wall and its bottom edge is three units above the highest
    /// rock in the room (or the floor, if there isn't one).
    fn add_shape(&mut self, shape: Shape) {
        let highest_y = dbg!(self.highest_rock_y());
        let mut points = Points(shape.starting_coords());
        points = dbg!(points);
        points.iter_mut().for_each(|p| {
            p.0 += 3;
            p.1 += highest_y + 4;
        });
        let entity = Entity { shape, points };
        self.entities.push(entity);
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
    fn test_board_add_shape_l() {
        let mut b = Board::new();
        assert_eq!(b.highest_rock_y(), 0);

        // |.......| 7
        // |....#..| 6
        // |....#..| 5
        // |..###..| 4
        // |.......| 3
        // |.......| 2
        // |.......| 1
        // +-------+
        b.add_shape(Shape::L);
        println!("{b}");
        assert_eq!(b.highest_rock_y(), 6);
        assert_eq!(
            b.sorted_points(),
            vec![
                Point(3, 4),
                Point(4, 4),
                Point(5, 4),
                Point(5, 5),
                Point(5, 6)
            ]
        );
    }

    #[test]
    fn test_board_add_shape_pipe() {
        let mut b = Board::new();
        assert_eq!(b.highest_rock_y(), 0);

        // |..#....| 7
        // |..#....| 6
        // |..#....| 5
        // |..#....| 4
        // |.......| 3
        // |.......| 2
        // |.......| 1
        // +-------+
        b.add_shape(Shape::Pipe);
        b = dbg!(b);
        assert_eq!(b.highest_rock_y(), 7);
        assert_eq!(
            b.sorted_points(),
            vec![Point(2, 7), Point(2, 6), Point(2, 5), Point(2, 4)]
        );
    }

    #[test]
    fn test_board_add_shape() {
        let mut b = Board::new();
        assert_eq!(b.highest_rock_y(), 0);

        // |..##...| 5
        // |..##...| 4
        // |.......| 3
        // |.......| 2
        // |.......| 1
        // +-------+
        let shape = Shape::Square;
        b.add_shape(shape);
        assert_eq!(b.highest_rock_y(), 5);
        assert_eq!(
            b.sorted_points(),
            vec![Point(2, 5), Point(3, 5), Point(2, 4), Point(3, 4)]
        );

        // |..#....| 12
        // |..#....|
        // |..#....| 10
        // |..#....|
        // |.......|
        // |.......|
        // |.......|
        // |..##...| 5
        // |..##...| 4
        // |.......|
        // |.......|
        // |.......| 1
        // +-------+
        // TODO: I don't think this is right.
        assert_eq!(b.highest_rock_y(), 5);
        let shape = Shape::Pipe;
        b.add_shape(shape);
        assert_eq!(b.highest_rock_y(), 12);
        assert_eq!(
            b.sorted_points(),
            vec![
                Point(2, 12),
                Point(2, 11),
                Point(2, 10),
                Point(2, 9),
                Point(2, 5),
                Point(3, 5),
                Point(2, 4),
                Point(3, 4)
            ]
        );
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

// the floor is at level y = 0. positions above
// the board are at y > 0.
#[derive(Debug)]
pub struct Board {
    width: usize, // always 7
    entities: Vec<Entity>,
}

type Shapes = Box<dyn Iterator<Item = Shape>>;
type Gusts = Box<dyn Iterator<Item = Gust>>;

#[derive(Debug)]
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
