use std::rc::Rc;

use crate::prelude::*;

#[derive(Clone)]
pub struct Solver {
    map: Rc<Map>,
    moves: usize,
    score: i64,
    valves: Valves,
    parent: Option<Rc<Solver>>,
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
        //println!("do_solve {depth} {}", self.valves);

        if self.moves == 0 || self.valves.all_open() {
            let multiple: i64 = self.moves.try_into().unwrap();
            self.score += self.valves.sum_open_rates() * (multiple + 1);
            println!(
                "Finished. score:{} moves:{}, depth:{}",
                self.score, self.moves, depth
            );
            return self.score;
        }

        // decrement the move counter b/c we're going to be moving.
        self.moves -= 1;

        // gather scores into this vec
        let mut scores = vec![];

        // if we can turn the valve open, do that.
        if self.valves.can_open_current() {
            let mut s = self.clone();
            s.valves.open_current();
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
    fn move_to(&mut self, name: &str) {
        self.valves.move_to(self.map.get(name));
    }
    pub fn new(args: &Args, map: Map) -> Self {
        let map = Rc::new(map);
        let moves = args.minutes;
        let score = 0;
        let current = map.get("AA");
        let valves = Valves::new(current, map.valves());
        let parent = None;
        Self {
            map,
            moves,
            score,
            valves,
            parent,
        }
    }
}
