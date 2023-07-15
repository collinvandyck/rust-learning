use std::{collections::HashSet, fmt::Display, ops::Deref};

use crate::prelude::*;

pub struct Rope {
    upper_left: Point,
    lower_right: Point,
    knots: Vec<NamePoint>,
    tail_visits: HashSet<Point>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        let mut knots = vec![];
        for i in (1..=num_knots - 1).rev() {
            knots.push(NamePoint::new(format!("{i}").as_str()));
        }
        knots.push(NamePoint::new("H"));
        knots = dbg!(knots);
        let origin = Point::zero();
        let mut res = Self {
            upper_left: origin,
            lower_right: origin,
            tail_visits: HashSet::new(),
            knots,
        };
        res.register_tail();
        res
    }
    pub fn exec(&mut self, mov: &Move) {
        for _ in 0..mov.amount {
            // new stuff
            self.move_knots(mov.direction);
            self.register_bounds();

            /*
            self.mov_head(mov.direction);
            self.register_bounds_for_point(self.head.point);
            //println!("Moved head {:?}\n{self}", mov.direction);
            self.mov_tail();
            self.register_bounds_for_point(self.tail.point);
            self.register_tail();
            //println!("Move complete tails:{}\n{self}", self.tail_visits());
            */
        }
    }
    // moves all of the knots, starting with the first knot at the end of the knots
    // vec ("H") and then processing each knot that exists before it.
    pub fn move_knots(&mut self, dir: Direction) {
        // move the head knot according to direction.
        if let Some(knot) = self.knots.last_mut() {
            println!("Moving knot {knot:?}");
            knot.point = match dir {
                Direction::Right => knot.point.combine(&Point(1, 0)),
                Direction::Left => knot.point.combine(&Point(-1, 0)),
                Direction::Up => knot.point.combine(&Point(0, 1)),
                Direction::Down => knot.point.combine(&Point(0, -1)),
            };
        }
        // move all of the following knots, one at a time.
        let length = self.knots.len();
        for hidx in (1..length).rev() {
            for tidx in (0..length - 1).rev() {
                let next = self.knots.get(hidx).unwrap().point;
                let tail = self.knots.get(tidx).unwrap().point;
                if let Some(next) = self.next_tail(&tail, &next) {
                    self.knots.get_mut(tidx).unwrap().point = next;
                }
            }
        }
    }
    pub fn tail_visits(&self) -> usize {
        self.tail_visits.len()
    }
    fn next_tail(&self, tail: &Point, next: &Point) -> Option<Point> {
        let difference = next.difference(tail);
        // if the difference is 2 in either up/left/down/right, adjust tail
        // by that amount
        match difference.abs() {
            Point(0, 2) => {
                // y has changed
                let adjust = &difference.combine(&Point(0, -1)).normalize();
                Some(tail.combine(adjust))
            }
            Point(2, 0) => {
                // x has changed
                let adjust = &difference.combine(&Point(-1, 0)).normalize();
                Some(tail.combine(adjust))
            }
            Point(0, 0) | Point(1, 0) | Point(0, 1) | Point(1, 1) => {
                // not enough of a difference to matter
                None
            }
            Point(_, _) => {
                // we must move the tail diagonally
                Some(tail.combine(&difference.normalize()))
            }
        }
    }
    /*
    fn mov_tail(&mut self) {
        let difference = self.head.difference(&self.tail);

        // if the difference is 2 in either up/left/down/right, adjust tail
        // by that amount
        match difference.abs() {
            Point(0, 2) => {
                // y has changed
                let adjust = &difference.combine(&Point(0, -1)).normalize();
                self.tail.point = self.tail.point.combine(adjust);
            }
            Point(2, 0) => {
                // x has changed
                let adjust = &difference.combine(&Point(-1, 0)).normalize();
                self.tail.point = self.tail.point.combine(adjust);
            }
            Point(0, 0) | Point(1, 0) | Point(0, 1) | Point(1, 1) => {
                // not enough of a difference to matter
            }
            Point(_, _) => {
                // we must move the tail diagonally
                self.tail.point = self.tail.point.combine(&difference.normalize());
            }
        };
    }
    */
    fn register_bounds(&mut self) {
        self.knots.iter().for_each(|knot| {
            let point = knot.point;
            self.upper_left = Point(
                i32::min(self.upper_left.0, point.0),
                i32::min(self.upper_left.1, point.1),
            );
            self.lower_right = Point(
                i32::max(self.lower_right.0, point.0),
                i32::max(self.lower_right.1, point.1),
            )
        });
    }
    fn register_tail(&mut self) {
        self.knots.get(0).iter().for_each(|k| {
            self.tail_visits.insert(k.point);
        })
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for y in (self.upper_left.1..=self.lower_right.1).rev() {
            for x in self.upper_left.0..=self.lower_right.0 {
                let point = Point::new(x, y);
                if let Some(np) = self.knots.iter().find(|p| p.point == point) {
                    buf.push_str(&np.name);
                } else {
                    buf.push_str(".");
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    // normalizes the point to an adjustment of at most 1 in any direction
    fn normalize(&self) -> Self {
        let mut x = self.0;
        let mut y = self.1;
        if x < -1 {
            x = -1;
        }
        if x > 1 {
            x = 1;
        }
        if y < -1 {
            y = -1;
        }
        if y > 1 {
            y = 1;
        }
        Self(x, y)
    }
}

#[test]
fn test_hashmap() {
    let mut hs = HashSet::new();
    let p = Point(0, 0);
    hs.insert(p);
    hs.insert(p);
    hs.insert(p);
    assert_eq!(1, hs.len());
}
