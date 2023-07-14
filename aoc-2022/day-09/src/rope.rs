use std::{fmt::Display, ops::Deref, slice};

use crate::prelude::*;

pub struct Rope {
    upper_left: Point,
    lower_right: Point,
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
            upper_left: Point::zero(),
            lower_right: Point::zero(),
        }
    }
    pub fn exec(&mut self, mov: &Move) {
        for _ in 0..mov.amount {
            self.mov_head(mov.direction);
            self.mov_tail();
            self.register_bounds(self.head.point);
            self.register_bounds(self.tail.point);
            println!("{self}");
        }
    }
    fn mov_head(&mut self, direction: Direction) {
        self.head.point = match direction {
            Direction::Right => self.head.combine(&Point(1, 0)),
            Direction::Left => self.head.combine(&Point(-1, 0)),
            Direction::Up => self.head.combine(&Point(0, 1)),
            Direction::Down => self.head.combine(&Point(0, -1)),
        };
    }
    fn mov_tail(&mut self) {
        let difference = self.head.difference(&self.tail);

        // if the difference is 2 in either up/left/down/right, adjust tail
        // by that amount
        match difference.abs() {
            Point(0, 2) => {
                // y has changed
                self.tail.point = self.tail.point.combine(&difference.combine(&Point(0, -1)));
            }
            Point(2, 0) => {
                // x has changed
                self.tail.point = self.tail.point.combine(&difference.combine(&Point(-1, 0)));
            }
            _ => {
                // not in easy mode
            }
        };
        println!("Difference: {difference:?}");
    }
    fn register_bounds(&mut self, point: Point) {
        self.upper_left = Point(
            i32::min(self.upper_left.0, point.0),
            i32::min(self.upper_left.1, point.1),
        );
        self.lower_right = Point(
            i32::max(self.lower_right.0, point.0),
            i32::max(self.lower_right.1, point.1),
        )
    }
    fn points(&self) -> [NamePoint; 3] {
        [self.head.clone(), self.tail.clone(), self.start.clone()]
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let points = self.points();
        let mut buf = String::new();
        for y in (self.upper_left.1..=self.lower_right.1).rev() {
            for x in self.upper_left.0..=self.lower_right.0 {
                let point = Point::new(x, y);
                if let Some(np) = points.iter().find(|p| p.point == point) {
                    buf.push_str(&np.name);
                } else {
                    buf.push_str("_");
                }
                buf.push_str(" ");
            }
            buf.push('\n');
        }
        write!(f, "{buf}")
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
            point: Point::zero(),
        }
    }
}

impl Deref for NamePoint {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
    fn zero() -> Self {
        Self(0, 0)
    }
    fn abs(&self) -> Self {
        Self(i32::abs(self.0), i32::abs(self.1))
    }
    fn combine(&self, other: &Point) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
    fn difference(&self, other: &Point) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
