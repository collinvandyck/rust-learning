use std::{collections::HashSet, rc::Rc};

use crate::prelude::*;

#[derive(Clone)]
pub struct Solver {
    map: Rc<Map>,
    moves: usize,
    score: i64,
    valves: Valves,
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
            self.valves.open().len(),
            self.valves.clos().len()
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
        self.valves.tunnels().for_each(|name| {
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
        let valves = Valves::new(current, map.valves());
        Self {
            map,
            moves,
            score,
            current,
            valves,
        }
    }
}
