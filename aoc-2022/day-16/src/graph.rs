use crate::prelude::*;
use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_network_conns() {
        let valves = Parser::read_file("example.txt");
        let net = Network::new(valves.clone().into_iter());
        let conns = net.connections("AA");
        assert_eq!(conns[&"DD".into()].0, vec![("AA".into(), "DD".into())]);
        assert_eq!(
            conns[&"CC".into()].0,
            vec![("AA".into(), "BB".into()), ("BB".into(), "CC".into())]
        );
        assert_eq!(conns.len(), valves.len() - 1);
    }
}

#[derive(Clone)]
pub struct State<'a> {
    net: &'a Network,
    position: Name,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    open_valves: HashSet<Name>,
    depth: usize,
}

impl<'a> State<'a> {
    pub fn new(net: &'a Network, position: Name) -> Self {
        Self {
            net,
            position,
            max_turns: 30,
            turn: 0,
            pressure: 0,
            depth: 0,
            open_valves: HashSet::default(),
        }
    }
    fn turns_left(&self) -> u64 {
        self.max_turns - self.turn
    }
    pub fn solve(&mut self) -> u64 {
        self.solve_recursive()
        //self.faster_than_lime_solution()
    }
    /*
    fn faster_than_lime_solution(&self) -> u64 {
        let (state, moves) = self.best_moves();
        for mov in &moves {
            println!("{mov}");
        }
        state.pressure
    }
    fn best_moves(&self) -> (Self, VecDeque<Move>) {
        let mut best_moves = VecDeque::new();
        let mut best_state = self.clone();
        for mov in self.moves() {
            let next = self.apply(&mov).clone();
            let (next, mut next_moves) = next.best_moves(); // recurse
            next_moves.push_front(mov);
            if next.pressure > best_state.pressure {
                best_moves = next_moves.clone();
                best_state = next.clone();
            }
        }
        (best_state.clone(), best_moves.clone())
    }
    */
    fn solve_recursive(&mut self) -> u64 {
        self.moves()
            .map(|mov| {
                let mut next = self.apply(&mov);
                next.solve_recursive()
            })
            .max()
            .unwrap_or(self.pressure)
    }
    fn apply(&self, mov: &Move) -> Self {
        let mut cloned = self.clone();
        cloned.depth += 1;
        cloned.pressure += mov.reward;
        cloned.turn += mov.cost();
        cloned.open_valves.insert(mov.target);
        cloned.position = mov.target;
        cloned
    }
    fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        let (_valve, conns) = &self.net.valves[&self.position];
        conns.into_iter().flat_map(|(target, (path, flow))| {
            if self.open_valves.contains(target) {
                return None;
            }
            if flow.0 == 0 {
                return None;
            }
            let turns_to_travel = path.len() as u64;
            let turns_to_open = 1_u64;
            let turns_total = turns_to_travel + turns_to_open;
            // the amount of time the valve will be on is the total number
            // of turns left subtracted by the time required to open it.
            let Some(turns) = self.turns_left().checked_sub(turns_total) else {
                    // we do not have the ability to make this move.
                    return None;
                };
            let reward = turns * flow.0;
            let mov = Move {
                reward,
                target: *target,
                path,
            };
            Some(mov)
        })
    }
    /*
    fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        self.net
            .connections(self.position)
            .into_iter()
            .flat_map(|(target, path)| {
                if self.open_valves.contains(&target) {
                    return None;
                }
                let flow = self.net.valves[&target].rate;
                if flow == 0 {
                    return None;
                }
                let turns_to_travel = path.len() as u64;
                let turns_to_open = 1_u64;
                let turns_total = turns_to_travel + turns_to_open;
                // the amount of time the valve will be on is the total number
                // of turns left subtracted by the time required to open it.
                let Some(turns) = self.turns_left().checked_sub(turns_total) else {
                    // we do not have the ability to make this move.
                    return None;
                };
                let reward = turns * flow;
                let mov = Move {
                    reward,
                    target,
                    path,
                };
                Some(mov)
            })
    }
    */
}

#[derive(Debug, Clone)]
pub struct Move<'a> {
    reward: u64, // the accumulative reward for making this move.
    target: Name,
    path: &'a Path,
}

impl Move<'_> {
    fn cost(&self) -> u64 {
        let cost_to_move = self.path.len() as u64;
        let cost_to_open = 1_u64;
        cost_to_move + cost_to_open
    }
}

impl Display for Move<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:?} cost:{} reward:{}",
            self.target,
            self.path,
            self.cost(),
            self.reward
        )
    }
}

// clearer semantics when used in tuples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flow(u64);

type Connections = HashMap<Name, (Path, Flow)>;

#[derive(Debug)]
pub struct Network {
    valves: HashMap<Name, (Valve, Connections)>,
}

pub type Path = Vec<(Name, Name)>;

impl Network {
    pub fn new(iter: impl Iterator<Item = Valve>) -> Self {
        let mut valves = HashMap::new();
        for valve in iter {
            valves.insert(valve.name, (valve, Connections::default()));
        }
        let mut net = Self { valves };
        let keys = net.valves.keys().copied().collect::<Vec<_>>();
        for key in keys {
            // type Connections = HashMap<Name, (Path, Flow)>;
            let conns = net.connections(key);
            net.valves.get_mut(&key).unwrap().1 = conns;
        }
        net
    }

    // from fasterthanlime's impl
    /*
    pub fn connections_lime<N>(&self, from: N) -> HashMap<Name, Path>
    where
        N: Into<Name>,
    {
        let from = from.into();
        let mut current: HashMap<Name, Path> = HashMap::default();
        current.insert(from, vec![]);
        let mut res = current.clone();
        while !current.is_empty() {
            let mut next: HashMap<Name, Path> = HashMap::default();
            for (name, path) in current {
                for link in self.valves[&name].links.iter().copied() {
                    if let Entry::Vacant(e) = res.entry(link) {
                        let conn_path: Path = path
                            .iter()
                            .copied()
                            .chain(std::iter::once((name, link)))
                            .collect();
                        e.insert(conn_path.clone());
                        next.insert(link, conn_path);
                    }
                }
            }
            current = next;
        }
        res
    }
    */

    /// Given a valve name, return a list of valves we can travel to, along
    /// with the path to get there.
    ///
    /// Only the shortest paths are considered, so the search ends.
    // type Connections = HashMap<Name, (Path, Flow)>;
    pub fn connections<N>(&self, from: N) -> HashMap<Name, (Path, Flow)>
    where
        N: Into<Name>,
    {
        let from = from.into();
        let mut res: HashMap<Name, (Path, Flow)> = HashMap::new();
        let mut queue: VecDeque<(Name, Path)> = VecDeque::new();
        queue.push_back((from, vec![]));

        let mut visited = HashSet::new();

        while let Some((name, path)) = queue.pop_front() {
            visited.insert(name);
            let valve: &Valve = &self.valves.get(&name).unwrap().0;
            for next in valve.links.iter() {
                if visited.contains(next) {
                    continue;
                }
                let next_valve = &self.valves.get(&next).unwrap().0;
                let next_flow = next_valve.rate;
                let mut next_path = path.clone();
                next_path.push((name, *next));
                let next_item = (next_path.clone(), Flow(next_flow));
                res.insert(*next, next_item);
                queue.push_back((*next, next_path));
            }
        }
        res
    }
}
