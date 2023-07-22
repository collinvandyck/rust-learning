use std::{collections::HashMap, fmt::Display, rc::Rc};

use crate::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_map;

    #[test]
    fn test_graph_nano() {
        run_graph_test("nano.txt");
    }

    #[test]
    fn test_graph_example() {
        run_graph_test("example.txt");
    }

    #[test]
    fn test_graph_input() {
        run_graph_test("input.txt");
    }

    fn run_graph_test(filename: &str) {
        let m = load_map(filename);
        let g = Graph::new(Rc::new(m));
        let paths = g.paths_from("AA");
        paths.vectors.values().for_each(|path| {
            println!("{path}");
            // foo;
        });
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
    pub fn paths_from<'a, 's, S>(&'a self, s: S) -> &'a Paths
    where
        S: Into<&'s str>,
    {
        let v = self.map.get(s.into());
        self.paths.get(&v).unwrap()
    }
}

// the paths from a start node to all of the other nodes.
#[derive(Debug)]
pub struct Paths {
    from: Rc<Valve>,
    vectors: HashMap<Rc<Valve>, Path>,
}

// path represents a reachable valve from another valve
#[derive(Debug)]
struct Path {
    from: Rc<Valve>,
    to: Rc<Valve>,
    steps: Vec<Rc<Valve>>,
}

impl Path {
    fn steps(&self) -> usize {
        self.steps.len()
    }
    // the rate of the destination step
    fn rate(&self) -> i32 {
        self.steps.last().unwrap().rate
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names = self
            .steps
            .iter()
            .map(|v| v.name.as_str())
            .collect::<Vec<&str>>()
            .join("->");
        let cost = self.steps();
        let rate = self.rate();
        write!(f, "[{cost:2}] [+{rate:2}] {}->{}", self.from.name, names)
    }
}

impl Paths {
    fn new(from: Rc<Valve>) -> Self {
        let paths = HashMap::new();
        Self {
            from,
            vectors: paths,
        }
    }
    fn add(&mut self, to: Rc<Valve>, steps: Vec<Rc<Valve>>) {
        self.vectors.insert(
            to.clone(),
            Path {
                from: self.from.clone(),
                to,
                steps,
            },
        );
    }
}

fn paths_for_valve(map: Rc<Map>, start: Rc<Valve>) -> Paths {
    let mut mapper = Mapper::new(map, start.clone());
    mapper.map();
    mapper.paths
}

// starts at a particular node, and maps out what is reachable from that node
struct Mapper {
    map: Rc<Map>,
    start: Rc<Valve>,
    visited: HashMap<Rc<Valve>, usize>,
    paths: Paths,
}

impl Mapper {
    fn map(&mut self) {
        self.do_map(self.start.clone(), &mut vec![]);
    }
    fn do_map(&mut self, cur: Rc<Valve>, steps: &mut Vec<Rc<Valve>>) {
        if !self.mark_visited(cur.clone(), steps.clone()) {
            // early exit since we've already been here.
            return;
        }
        for tun in cur.tunnels.iter() {
            let next = self.map.get(tun);
            steps.push(next.clone());
            self.do_map(next, steps);
            steps.pop();
        }
    }
    // returns true if the node was not previously visited, or was visited with a higher cost
    fn mark_visited(&mut self, cur: Rc<Valve>, steps: Vec<Rc<Valve>>) -> bool {
        match self.visited.get(&cur) {
            Some(cost) if *cost < steps.len() => return false,
            _ => {}
        }
        self.visited.insert(cur.clone(), steps.len());
        if !steps.is_empty() {
            self.paths.add(cur, steps)
        }
        true
    }
    fn new(map: Rc<Map>, start: Rc<Valve>) -> Self {
        let visited = HashMap::new();
        let paths = Paths::new(start.clone());
        Self {
            map,
            start,
            visited,
            paths,
        }
    }
}
