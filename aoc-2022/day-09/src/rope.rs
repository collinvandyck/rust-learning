use std::{fmt::Display, ops::Deref};

use crate::prelude::*;

pub struct Rope {
    start: NamePoint,
    head: NamePoint,
    tail: NamePoint,
}

impl Rope {
    pub fn new() -> Self {
        Self {
            start: NamePoint::new("s"),
            head: NamePoint::new("H"),
            tail: NamePoint::new("T"),
        }
    }
    pub fn exec(&mut self, mov: &Move) {
        self.mov_head(mov);
        self.mov_tail();
    }
    fn mov_head(&mut self, mov: &Move) {
        self.head.point = match mov.direction {
            Direction::Right => self.head.combine(&Point(1, 0)),
            Direction::Left => self.head.combine(&Point(-1, 0)),
            Direction::Up => self.head.combine(&Point(0, -1)),
            Direction::Down => self.head.combine(&Point(0, 1)),
        };
    }
    fn mov_tail(&mut self) {}
    fn points(&self) -> [Point; 3] {
        [self.start.point, self.head.point, self.tail.point]
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get the min and max points
        let mut upper_left = Point::new();
        let mut lower_right = Point::new();
        for point in self.points() {
            upper_left = Point(
                i32::min(upper_left.0, point.0),
                i32::min(upper_left.1, point.1),
            );
            lower_right = Point(
                i32::max(lower_right.0, point.0),
                i32::max(lower_right.1, point.1),
            );
        }
        let mut buf = String::new();
        for row in 0..=lower_right.1 - upper_left.1 {
            for col in 0..=lower_right.0 - upper_left.0 {
                println!("{row}x{col}")
            }
        }
        write!(f, "{:?}", (upper_left, lower_right))
    }
}

#[derive(Debug, Clone)]
pub struct NamePoint {
    name: String,
    point: Point,
}

impl NamePoint {
    fn new(s: &str) -> Self {
        Self {
            name: s.to_string(),
            point: Point::new(),
        }
    }
}

impl Deref for NamePoint {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.point
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
