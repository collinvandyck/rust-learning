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
    pub fn solve(&mut self) -> i64 {
        self.do_solve(0)
    }
    // TODO:
    //  - There should be a way of assigning a score to a particular room.
    //    The score would let other recursions to exit early if they visited
    //    a room where the valve state was the same but the score was higher.
    //
    //    So this means we'd probably need a way to represent valve state.
    //    We already have the open/closed hashsets. If we could compare them
    //    along with the score, that might be enough to avoid doing work.
    fn do_solve(&mut self, depth: usize) -> i64 {
        println!(
            "do_solve {depth} open:{} closed:{}",
            self.open.len(),
            self.closed.len()
        );
        // start the turn, and update the score
        self.score += self.open_valve_rate_sum();

        // if we have no more moves, we are done
        if self.moves == 0 {
            return self.score;
        }
        // if all of the valves are open, then we can just simulate
        // the passage of time and return the modified score.
        if self.all_valves_open() {
            println!("All valves open");
            let multiple: i64 = self.moves.try_into().unwrap();
            println!("Multiple: {multiple}");
            self.score += self.open_valve_rate_sum() * multiple;
            return self.score;
        }

        // decrement the move counter b/c we're going to be moving.
        self.moves -= 1;

        // gather scores into this vec
        let mut scores = vec![];

        // if we can turn the valve open, do that.
        if self.can_open_valve() {
            let mut s = self.clone();
            s.open_valve();
            scores.push(s.do_solve(depth + 1));
        }
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
    fn open_valve_rate_sum(&self) -> i64 {
        self.open.iter().map(|v| v.rate).sum::<i32>() as i64
    }
    fn all_valves_open(&self) -> bool {
        self.closed.is_empty()
    }
    fn any_valves_closed(&self) -> bool {
        !self.closed.is_empty()
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

    #[derive(PartialEq, Eq, Debug)]
    struct State {
        open: HashSet<Rc<Valve>>,
        clos: HashSet<Rc<Valve>>,
    }

    impl State {}

    #[test]
    fn test_saved_state_super_struct() {
        let v1 = Rc::new(Valve::new("AA", 5, vec!["CC".to_string()]));
        let v2 = Rc::new(Valve::new("BB", 5, vec![]));
        let v3 = Rc::new(Valve::new("CC", 5, vec![]));

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s1 = State { open, clos: closed };

        let open: HashSet<Rc<Valve>> = [&v1, &v2].into_iter().map(|s| s.clone()).collect();
        let closed: HashSet<Rc<Valve>> = [&v3].into_iter().map(|s| s.clone()).collect();
        let mut s2 = State { open, clos: closed };

        assert_eq!(s1, s2);

        s1.open.insert(v3.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v3.clone());
        assert_eq!(s1, s2);
        assert_eq!(s1, s2);

        s1.clos.clear();
        s1.open.clear();
        s2.clos.clear();
        s2.open.clear();
        assert_eq!(s1, s2);

        s1.open.insert(v1.clone());
        s2.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s1.clos.insert(v1.clone());
        assert_ne!(s1, s2);
        s2.open.insert(v1.clone());
        assert_eq!(s1, s2);
    }

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
