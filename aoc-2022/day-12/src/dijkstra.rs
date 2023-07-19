use std::collections::HashSet;

use crate::{Map, Point, Solver};

pub struct Dijkstra {
    map: Map,
    unvisited: HashSet<Node>,
    current: Node,
}

impl Solver for Dijkstra {
    fn solve(&mut self) -> Option<Vec<crate::Point>> {
        None
    }
}

impl Dijkstra {
    pub fn new(map: Map) -> Self {
        let current = Node::new(map.start, 0);
        let mut unvisited = HashSet::new();
        for row in 0..map.rows {
            for col in 0..map.cols {
                let point = Point(row, col);
                if point != map.start {
                    let node = Node::new(point, i64::MAX);
                    unvisited.insert(node);
                }
            }
        }
        Self {
            map,
            unvisited,
            current,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    point: Point,
    distance: i64,
}

impl Node {
    fn new(point: Point, distance: i64) -> Self {
        Self { point, distance }
    }
}
