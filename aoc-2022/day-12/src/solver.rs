use std::{collections::HashSet, sync::Arc};

use crate::prelude::*;

pub struct Solver {
    map: Arc<Map>,
    start: Point,
    finish: Point,
    path: Vec<Point>,
    visited: HashSet<Point>,
}

impl Solver {
    pub fn new(map: Arc<Map>, start: Point, finish: Point) -> Self {
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
    pub fn solve(&self) {}
}
