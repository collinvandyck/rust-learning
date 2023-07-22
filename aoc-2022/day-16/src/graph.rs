use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_map;

    #[test]
    fn test_graph_nano() {
        let m = load_map("nano.txt");
        let g = Graph::new(Rc::new(m));
    }
}

const START: &'static str = "AA";

pub struct Graph {
    map: Rc<Map>,
    paths: HashMap<Rc<Valve>, Paths>,
}

impl Graph {
    pub fn new(map: Rc<Map>) -> Self {
        let mut paths = HashMap::new();
        for valve in map.valves() {
            let vp = paths_for_valve(map.clone(), valve.clone());
            paths.insert(valve.clone(), vp);
        }
        Self { map, paths }
    }
}

// the paths from a start node to all of the other nodes.
struct Paths {
    from: Rc<Valve>,
    paths: HashMap<Rc<Valve>, Path>,
}

// path represents a reachable valve from another valve
struct Path {
    to: Rc<Valve>,
    steps: Vec<Rc<Valve>>,
}

fn paths_for_valve(map: Rc<Map>, start: Rc<Valve>) -> Paths {
    let mut mapper = Mapper::new(map, start.clone());
    mapper.map();
    Paths {
        from: start,
        paths: HashMap::new(),
    }
}

// starts at a particular node, and maps out what is reachable from that node
struct Mapper {
    map: Rc<Map>,
    start: Rc<Valve>,
    visited: HashSet<Rc<Valve>>,
}

impl Mapper {
    fn map(&mut self) {
        println!("Mapping node {}", self.start.name);
        self.do_map(self.start.clone(), vec![]);
    }

    fn do_map(&mut self, cur: Rc<Valve>, buf: Vec<Rc<Valve>>) {}

    fn visited() {}

    fn new(map: Rc<Map>, start: Rc<Valve>) -> Self {
        let visited = HashSet::new();
        Self {
            map,
            start,
            visited,
        }
    }
}
