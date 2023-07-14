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
        println!("mov head #{mov}")
    }
    fn mov_tail(&mut self) {
        println!("mov tail")
    }
}

pub struct Point(i32, i32);

impl Point {
    fn new() -> Self {
        Self(0, 0)
    }
}
