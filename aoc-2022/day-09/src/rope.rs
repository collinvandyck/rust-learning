use std::fmt::Display;

use crate::prelude::*;

pub struct Rope {
    start: Point,
    head: Point,
    tail: Point,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            start: Point::new(),
            head: Point::new(),
            tail: Point::new(),
        }
    }
    pub fn exec(&mut self, mov: &Move) {
        self.mov_head(mov);
        self.mov_tail();
    }
    fn mov_head(&mut self, mov: &Move) {
        self.head = match mov.direction {
            Direction::Right => self.head.combine(&Point(1, 0)),
            Direction::Left => self.head.combine(&Point(-1, 0)),
            Direction::Up => self.head.combine(&Point(0, -1)),
            Direction::Down => self.head.combine(&Point(0, 1)),
        };
        println!("mov head #{mov} now is {:?}", self.head);
    }
    fn mov_tail(&mut self) {
        println!("mov tail")
    }
    fn points(&self) -> [Point; 3] {
        [self.start, self.head, self.tail]
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get the min and max points
        let mut upper_left = Point::new();
        let mut lower_right = Point::new();
        for point in self.points() {
            println!(
                "Looking at point {point:?} upper_left:{upper_left:?} lower_right:{lower_right:?}"
            );
            upper_left = Point(
                i32::min(upper_left.0, point.0),
                i32::min(upper_left.1, point.1),
            );
            lower_right = Point(
                i32::max(lower_right.0, point.0),
                i32::max(lower_right.1, point.1),
            );
        }
        write!(f, "{:?}", (upper_left, lower_right))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point(i32, i32);

impl Point {
    fn new() -> Self {
        Self(0, 0)
    }
    fn combine(&self, other: &Point) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
