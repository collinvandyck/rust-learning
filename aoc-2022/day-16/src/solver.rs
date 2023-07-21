use std::{collections::HashMap, rc::Rc};

use crate::prelude::*;

#[derive(Clone)]
pub struct Solver {
    map: Rc<Map>,
    open: HashMap<String, i32>,
    moves: i32,
    score: i64,
    current: Rc<Valve>,
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
        // update the score
        self.score += self.open.values().sum::<i32>() as i64;

        // if we have no more moves, we are done
        if self.moves == 0 {
            println!("Score: {} Depth: {}", self.score, depth);
            return self.score;
        }

        // decrement the move counter
        self.moves -= 1;

        println!("Moves: {}", self.moves);

        // we will accumulate scores here
        let mut scores = vec![];

        // if we can turn the valve open, do that.
        if !self.valve_open() {
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
        *scores.iter().max().unwrap()
    }

    fn move_to(&mut self, name: &str) {
        self.current = self.map.get(name);
    }

    fn open_valve(&mut self) {
        let name = self.current.name.to_string();
        self.open.insert(name, self.current.rate);
    }

    // returns true if the current valve is open
    fn valve_open(&self) -> bool {
        self.open.contains_key(&self.current.name)
    }

    pub fn new(map: Map) -> Self {
        let map = Rc::new(map);
        let open = HashMap::new();
        let moves = 30;
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
