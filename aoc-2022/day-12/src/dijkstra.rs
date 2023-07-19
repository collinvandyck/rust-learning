use crate::{Map, Solver};

pub struct Dijkstra {
    map: Map,
}

impl Solver for Dijkstra {
    fn solve(&mut self) -> Option<Vec<crate::Point>> {
        None
    }
}

impl Dijkstra {
    fn new(map: Map) -> Self {
        Self { map }
    }
}
