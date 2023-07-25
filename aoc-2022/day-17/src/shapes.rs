use crate::prelude::*;

pub struct Iter {
    shapes: [Shape; 5],
    idx: usize,
}

impl Iterator for Iter {
    type Item = Shape;
    fn next(&mut self) -> Option<Self::Item> {
        let shape = self.shapes[self.idx];
        self.idx = (self.idx + 1) % self.shapes.len();
        Some(shape)
    }
}

impl IntoIterator for Shapes {
    type Item = Shape;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            shapes: self.0,
            idx: 0,
        }
    }
}

pub struct Shapes([Shape; 5]);

pub fn shapes() -> Shapes {
    Shapes([
        Shape::Slab,
        Shape::Cross,
        Shape::L,
        Shape::Pipe,
        Shape::Square,
    ])
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Slab,
    Cross,
    L,
    Pipe,
    Square,
}

impl Shape {
    // returns the positions of the rocks in the shape relative
    // to a grid starting at (0,0).
    pub fn starting_coords(&self) -> Vec<Point> {
        match self {
            Shape::Slab => vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)],
            Shape::Cross => vec![
                Point(1, 0),
                Point(0, 1),
                Point(1, 1),
                Point(2, 1),
                Point(1, 2),
            ],
            Shape::L => vec![
                Point(2, 0),
                Point(2, 1),
                Point(0, 2),
                Point(1, 2),
                Point(2, 2),
            ],
            Shape::Pipe => vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)],
            Shape::Square => vec![Point(0, 0), Point(1, 0), Point(0, 1), Point(1, 1)],
        }
    }
    pub fn height(&self) -> i32 {
        match self {
            Shape::Slab => 1,
            Shape::Cross => 3,
            Shape::L => 3,
            Shape::Pipe => 4,
            Shape::Square => 2,
        }
    }
}

#[test]
fn test_shapes_iter() {
    let mut iter = shapes().into_iter();
    assert_eq!(iter.next(), Some(Shape::Slab));
    assert_eq!(iter.next(), Some(Shape::Cross));
    assert_eq!(iter.next(), Some(Shape::L));
    assert_eq!(iter.next(), Some(Shape::Pipe));
    assert_eq!(iter.next(), Some(Shape::Square));
    assert_eq!(iter.next(), Some(Shape::Slab));
    assert_eq!(iter.next(), Some(Shape::Cross));
    assert_eq!(iter.next(), Some(Shape::L));
    assert_eq!(iter.next(), Some(Shape::Pipe));
    assert_eq!(iter.next(), Some(Shape::Square));
}
