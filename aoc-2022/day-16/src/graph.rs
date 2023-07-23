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
        assert_eq!(conns[&"DD".into()], vec![("AA".into(), "DD".into())]);
        assert_eq!(
            conns[&"CC".into()],
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
    }
    fn faster_than_lime_solution(&self) -> u64 {
        let (state, moves) = self.best_moves();
        state.pressure
    }
    fn best_moves(&self) -> (Self, Vec<Move>) {
        let mut best_moves = vec![];
        let mut best_state = self.clone();
        let mut best_pressure = 0;

        let mut next_moves = self.moves();
        next_moves.sort_by_key(|m| m.reward);
        next_moves.reverse();

        for mov in next_moves {
            let next = self.apply(&mov);
            let (next, mut next_moves) = next.best_moves();
            next_moves.push(mov);
            if next.pressure > best_pressure {
                best_pressure = next.pressure;
                best_moves = next_moves;
                best_state = next;
            }
        }
        (best_state, best_moves)
    }
    fn solve_recursive(&mut self) -> u64 {
        self.moves()
            .into_iter()
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
    // returns possible moves from the current postition
    pub fn moves(&self) -> Vec<Move> {
        let mut res = vec![];
        let conns: HashMap<Name, Path> = self.net.connections(self.position);
        for (target, path) in conns {
            if self.open_valves.contains(&target) {
                continue;
            }
            let flow = self.net.valves[&target].rate;
            if flow == 0 {
                continue;
            }
            let turns_to_travel = path.len() as u64;
            let turns_to_open = 1_u64;
            let turns_total = turns_to_travel + turns_to_open;
            // the amount of time the valve will be on is the total number
            // of turns left subtracted by the time required to open it.
            let Some(turns) = self.turns_left().checked_sub(turns_total) else {
                // we do not have the ability to make this move.
                continue;
            };
            let reward = turns * flow;
            let mov = Move {
                reward,
                target,
                path,
            };
            res.push(mov);
        }
        res
    }
}

pub struct Move {
    reward: u64, // the accumulative reward for making this move.
    target: Name,
    path: Path,
}

impl Move {
    fn cost(&self) -> u64 {
        let cost_to_move = self.path.len() as u64;
        let cost_to_open = 1_u64;
        cost_to_move + cost_to_open
    }
}

impl Display for Move {
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

    // from fasterthanlime's impl
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

    /// Given a valve name, return a list of valves we can travel to, along
    /// with the path to get there.
    ///
    /// Only the shortest paths are considered, so the search ends.
    pub fn connections<N>(&self, from: N) -> HashMap<Name, Path>
    where
        N: Into<Name>,
    {
        let from = from.into();
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
