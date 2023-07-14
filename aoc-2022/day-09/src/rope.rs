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
}

#[derive(Debug)]
pub struct Point(i32, i32);

impl Point {
    fn new() -> Self {
        Self(0, 0)
    }
    fn combine(&self, other: &Point) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
