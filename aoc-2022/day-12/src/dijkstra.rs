use std::collections::{HashMap, HashSet};

use crate::{Map, Point, Solver};

pub struct Dijkstra {
    map: Map,
    unvisited: HashMap<Point, Node>,
    current: Node,
}

impl Solver for Dijkstra {
    fn solve(&mut self) -> Option<Vec<crate::Point>> {
        // get the next unvisited nodes from where we are
        let nexts = self
            .map
            .next_moves_from(&self.current.point)
            .into_iter()
            .flatten()
            .filter(|p| self.unvisited.contains_key(p))
            .collect::<Vec<_>>();

        for next in nexts {
            let node = self.unvisited.get_mut(&next).unwrap();
            dbg!(node);
        }

        None
    }
}

impl Dijkstra {
    pub fn new(map: Map) -> Self {
        let current = Node::new(map.start, 0);
        let mut unvisited = HashMap::new();
        for row in 0..map.rows {
            for col in 0..map.cols {
                let point = Point(row, col);
                if point != map.start {
                    let node = Node::new(point, i64::MAX);
                    unvisited.insert(point, node);
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
