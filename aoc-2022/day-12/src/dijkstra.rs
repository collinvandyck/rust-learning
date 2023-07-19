use std::collections::{HashMap, HashSet};

use crate::{Map, Point, Solver};

pub struct Dijkstra {
    map: Map,
    unvisited: HashMap<Point, Node>,
    visited: HashSet<Node>,
    current: Point,
}

impl Solver for Dijkstra {
    fn solve(&mut self) -> Option<Vec<crate::Point>> {
        // get the current node
        let current_distance = self.unvisited.get(&self.current).unwrap().distance;

        // get the next unvisited nodes from where we are
        let nexts = self
            .map
            .next_moves_from(&self.current)
            .into_iter()
            .flatten()
            .filter(|p| self.unvisited.contains_key(p))
            .collect::<Vec<_>>();

        for next in nexts {
            let node: &mut Node = self.unvisited.get_mut(&next).unwrap();
            let distance = current_distance + 1;
            // set the new distance on the node if it's shorter
            if distance < node.distance {
                node.distance = distance;
            }
            dbg!(node);
        }

        let removed = self.unvisited.remove(&self.current).unwrap();
        self.visited.insert(removed);
        dbg!(removed);

        None
    }
}

impl Dijkstra {
    pub fn new(map: Map) -> Self {
        let current = map.start;
        let mut unvisited = HashMap::new();
        for row in 0..map.rows {
            for col in 0..map.cols {
                let point = Point(row, col);
                let node = if point == map.start {
                    Node::new(point, 1)
                } else {
                    Node::new(point, i64::MAX)
                };
                unvisited.insert(point, node);
            }
        }
        let visited = HashSet::new();
        Self {
            map,
            unvisited,
            current,
            visited,
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
