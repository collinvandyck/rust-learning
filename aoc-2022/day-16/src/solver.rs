use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::prelude::*;

#[derive(Clone)]
pub struct Solver {
    map: Rc<Map>,
    open: HashMap<String, i32>,
    moves: usize,
    score: i64,
    current: Rc<Valve>,
}

#[test]
fn test_hashset_with_rc() {
    let mut hs: HashSet<Rc<Valve>> = HashSet::new();
    let v1 = Rc::new(Valve::new("AA", 5, vec![]));
    let v2 = Rc::new(Valve::new("BB", 5, vec![]));
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
        self.score += self.open.values().sum::<i32>() as i64;

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

    fn move_to(&mut self, name: &str) {
        self.current = self.map.get(name);
    }

    fn open_valve(&mut self) {
        let name = self.current.name.to_string();
        self.open.insert(name, self.current.rate);
    }

    fn can_open_valve(&self) -> bool {
        !self.open.contains_key(&self.current.name)
    }

    pub fn new(args: &Args, map: Map) -> Self {
        let map = Rc::new(map);
        let open = HashMap::new();
        let moves = args.minutes;
        let score = 0;
        let current = map.get("AA");
        Self {
            map,
            open,
            moves,
            score,
            current,
        }
    }
}
