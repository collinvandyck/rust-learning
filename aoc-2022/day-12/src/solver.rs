use std::{collections::HashSet, sync::Arc};

use crate::prelude::*;

// todo: considering using <arc<mutex<type holding success/failure data>>
// to prevent unnecessary work.
//
// Inner type is used as
//      let i = Arc::new(Mutex::new(inner));
//      ( Left { i.clone() }, Right { i.clone() })

#[derive(Clone)]
pub struct Solver {
    map: Arc<Map>,
    start: Point,
    finish: Point,
    path: Vec<Point>,
    visited: HashSet<Point>,
}

impl Solver {
    pub fn solve(&mut self) {}
}

impl Solver {
    pub fn new(map: Arc<Map>, start: Point, finish: Point) -> Self {
        "".split(jk)
        let path = vec![];
        let visited = HashSet::new();
        Self {
            map,
            start,
            finish,
            path,
            visited,
        }
    }
}
