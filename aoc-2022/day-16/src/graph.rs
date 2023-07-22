use crate::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Network {
    valves: HashMap<Name, Valve>,
}

pub type Path = Vec<(Name, Name)>;

impl Network {
    pub fn new(iter: impl Iterator<Item = Valve>) -> Self {
        let mut valves = HashMap::new();
        for valve in iter {
            valves.insert(valve.name, valve);
        }
        Self { valves }
    }

    /// Given a valve name, return a list of valves we can travel to, along
    /// with the path to get there.
    ///
    /// Only the shortest paths are considered, so the search ends.
    pub fn connections(&self, from: Name) -> HashMap<Name, Path> {
        let mut res = HashMap::new();

        let mut queue: VecDeque<(Name, Path)> = VecDeque::new();
        queue.push_back((from, vec![]));

        let mut visited = HashSet::new();

        while let Some((name, path)) = queue.pop_front() {
            visited.insert(name);
            let valve: &Valve = self.valves.get(&name).unwrap();
            for next in valve.links.iter() {
                if visited.contains(next) {
                    continue;
                }
                // need to build the new path
                let mut next_path = path.clone();
                next_path.push((name, *next));
                res.insert(*next, next_path.clone());
                queue.push_back((*next, next_path));
            }
        }
        res
    }
}
