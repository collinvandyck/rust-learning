use std::collections::{HashMap, HashSet, VecDeque};

use crate::{Map, Point, Solver};

pub struct Dijkstra {
    map: Map,
    unvisited: HashMap<Point, Node>,
    visited: HashMap<Point, Node>,
}

impl Solver for Dijkstra {
    fn solve(&mut self, start: Point) -> Option<Vec<Point>> {
        let mut iterations = 0;
        self.unvisited.get_mut(&start).unwrap().distance = 1;
        while let Some(cur_point) = self.next_point() {
            iterations += 1;
            let current = self.unvisited.remove(&cur_point).unwrap();
            // get the next unvisited nodes from where we are
            let nexts = self
                .map
                .next_moves_from(&cur_point)
                .into_iter()
                .flatten()
                .filter(|p| self.unvisited.contains_key(p))
                .collect::<Vec<_>>();
            for next in nexts {
                let node: &mut Node = self.unvisited.get_mut(&next).unwrap();
                let distance = current.distance + 1;
                // set the new distance on the node if it's shorter
                if distance < node.distance {
                    node.distance = distance;
                }
            }
            self.visited.insert(current.point, current);
            if current.point == self.map.finish {
                return Some(self.return_path(start));
            }
        }
        None
    }
}

impl Dijkstra {
    // if successfully completed, return the shortest path from the start to finish
    fn return_path(&self, start: Point) -> Vec<Point> {
        let mut res = VecDeque::new();
        let mut cur = self.map.finish;
        res.push_front(cur);
        while cur != start {
            cur = self
                .map
                .next_moves_to(&cur)
                .into_iter()
                .flatten()
                .map(|p| self.visited.get(&p))
                .flatten()
                .reduce(|n1, n2| if n1.distance < n2.distance { n1 } else { n2 })
                .map(|n| n.point)
                .unwrap();
            res.push_front(cur);
        }
        res.into()
    }
    // find the smallest distance node in the unvisited set
    fn next_point(&self) -> Option<Point> {
        self.unvisited
            .iter()
            .filter(|(point, node)| node.distance < i64::MAX)
            .map(|(p1, n1)| n1)
            .reduce(|n1, n2| if n1.distance < n2.distance { n1 } else { n2 })
            .map(|n| n.point)
    }
    pub fn new(map: Map) -> Self {
        let mut unvisited = HashMap::new();
        for row in 0..map.rows {
            for col in 0..map.cols {
                let point = Point(row, col);
                let node = Node::new(point, i64::MAX);
                unvisited.insert(point, node);
            }
        }
        let visited = HashMap::new();
        Self {
            map,
            unvisited,
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
