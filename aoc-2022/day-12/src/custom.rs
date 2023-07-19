use crate::{prelude::*, Map, Point, Solver};

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    slice::SliceIndex,
    str::FromStr,
    time::Instant,
};

pub struct Custom {
    map: Map,
    iterations: u64,
    visits: HashMap<Point, usize>, // point, depth
    min_solve: Option<usize>,
    short_circuits: u64,
}

impl Solver for Custom {
    fn solve(&mut self, start: Point) -> Option<Vec<Point>> {
        let path = vec![start];
        let visited = HashSet::from([start]);
        let res = self.do_solve(0, path, visited);
        println!("Short circuits: {}", self.short_circuit_percentage());
        println!("Iterations: {}", self.iterations);
        res
    }
}

impl Custom {
    pub fn new(map: Map) -> Self {
        let iterations = 0;
        let visits = HashMap::new();
        let min_solve = None;
        let short_circuits = 0;
        Self {
            map,
            iterations,
            visits,
            min_solve,
            short_circuits,
        }
    }
    #[allow(clippy::cast_precision_loss)]
    fn short_circuit_percentage(&self) -> String {
        if self.short_circuits == 0 {
            "none".to_string()
        } else {
            let short_circuits = self.short_circuits as f64;
            let iteration = self.iterations as f64;
            let short_circuits = short_circuits / iteration * 100.0;
            format!("{short_circuits:.2}%")
        }
    }
    // registers the current point as having been visited, and returns
    // true if the traversal should continue. if the point has not been
    // seen before true is returned. if the point has been visited before
    // but at a greater depth, true is returned so that we can find a more
    // optimal path. otherwise false is returned.
    //
    // In the case where we have seen the point before, but at the same
    // depth, there is no point in retracing the same steps so we return false.
    fn register_visit(&mut self, p: &Point, depth: usize) -> bool {
        match self.visits.entry(*p) {
            Entry::Occupied(mut e) => {
                let existing = e.get_mut();
                if *existing > depth {
                    *existing = depth;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(e) => {
                e.insert(depth);
                true
            }
        }
    }
    fn register_min_solve(&mut self, size: usize) {
        match self.min_solve {
            None => self.min_solve = Some(size),
            Some(existing) if size < existing => self.min_solve = Some(size),
            _ => {}
        }
    }
    fn do_solve(
        &mut self,
        depth: usize,
        mut path: Vec<Point>,
        mut visited: HashSet<Point>,
    ) -> Option<Vec<Point>> {
        self.iterations += 1;
        let current = path.last().unwrap();

        // are we done?
        if current == &self.map.finish {
            self.register_min_solve(depth);
            return Some(path);
        }
        // short circuit if we are too deep
        if let Some(min_solve) = self.min_solve {
            //println!("depth: {}, min_solve: {}", depth, min_solve);
            if depth >= min_solve {
                //println!("short circuit");
                self.short_circuits += 1;
                return None;
            }
        }

        // we are not done. mark the current node as being visited.
        if !self.register_visit(current, depth) {
            // we have already visited this node at this depth or greater. there
            // is no point in continuing.
            return None;
        }

        // generate the next moves
        let nexts = self.map.next_moves_from(current);

        let mut res: Option<Vec<Point>> = None;
        for next in nexts.into_iter().flatten() {
            //
            // if we have already seen the next node, don't bother.
            if visited.contains(&next) {
                continue;
            }

            // clone path and push the next node onto it.
            let mut path = path.clone();
            path.push(next);

            let visited = visited.clone();
            let next_res = self.do_solve(depth + 1, path, visited);
            if let Some(next_res) = next_res {
                res = match res {
                    None => Some(next_res),
                    Some(existing) if next_res.len() < existing.len() => Some(next_res),
                    _ => res,
                }
            }
        }
        res
    }
}
