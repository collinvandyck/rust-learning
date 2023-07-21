use std::{collections::HashSet, rc::Rc};

use crate::prelude::*;

#[derive(Clone)]
pub struct Solver {
    map: Rc<Map>,
    moves: usize,
    score: i64,
    current: Rc<Valve>,
    open: HashSet<Rc<Valve>>,
    closed: HashSet<Rc<Valve>>,
}

impl Solver {
    // solve returns the highest score
    //
    // Each move taken:
    //
    // First adds to the score any currently open valves.
    // Then one may choose to do one of the following:
    // - open the valve you are at
    // - move to another valve
    //
    pub fn solve(&mut self) -> i64 {
        self.do_solve(0)
    }
    fn do_solve(&mut self, depth: usize) -> i64 {
        self.score += self.open.iter().map(|v| v.rate).sum::<i32>() as i64;

        // if we have no more moves, we are done
        if self.moves == 0 {
            return self.score;
        }

        // decrement the move counter
        self.moves -= 1;
        let mut scores = vec![];

        // if we can turn the valve open, do that.
        if self.can_open_valve() {
            let mut s = self.clone();
            s.open_valve();
            scores.push(s.do_solve(depth + 1));
        }

        // if there are any more valves that are closed, keep going.

        // then try moving through each tunnel.
        self.current.tunnels.iter().for_each(|name| {
            let mut s = self.clone();
            s.move_to(name);
            scores.push(s.do_solve(depth + 1));
        });

        // return the max score
        let res = *scores.iter().max().unwrap();
        res
    }

    fn all_valves_open(&self) -> bool {
        !self.any_valves_closed()
    }
    fn any_valves_closed(&self) -> bool {
        self.closed.is_empty()
    }
    fn move_to(&mut self, name: &str) {
        self.current = self.map.get(name);
    }
    fn open_valve(&mut self) {
        let is_new = self.open.insert(self.current.clone());
        assert!(is_new);
        let existed = self.closed.remove(&self.current);
        assert!(existed);
    }
    fn can_open_valve(&self) -> bool {
        !self.open.contains(&self.current)
    }
    pub fn new(args: &Args, map: Map) -> Self {
        let map = Rc::new(map);
        let moves = args.minutes;
        let score = 0;
        let current = map.get("AA");
        let mut open = HashSet::new();
        let mut closed = HashSet::new();
        for valve in map.valves() {
            if valve.rate == 0 {
                open.insert(valve);
            } else {
                closed.insert(valve);
            }
        }
        Self {
            map,
            moves,
            score,
            current,
            open,
            closed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hashset_with_rc() {
        let mut hs: HashSet<Rc<Valve>> = HashSet::new();
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));

        // insertions
        assert!(!hs.contains(&v1));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        hs.insert(v1.clone());
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.insert(v2.clone());
        assert!(hs.contains(&v1));
        assert!(hs.contains(&v2));
        assert_eq!(2, hs.len());

        // deletions
        hs.remove(&v2);
        assert!(hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(1, hs.len());
        hs.remove(&v1);
        assert!(!hs.contains(&v1));
        assert!(!hs.contains(&v2));
        assert_eq!(0, hs.len());
    }
}
