use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Read, Result},
};

use crate::prelude::*;

// gyahhhhh
type IterResult = io::Result<String>;
type Iterable = Box<dyn Iterator<Item = IterResult>>;

// BetterIterator is an iterator that composes other iterators and
// consumes them in order.
pub struct BetterIterator(VecDeque<Iterable>);

impl BetterIterator {
    pub fn new(args: &Args) -> CatResult<BetterIterator> {
        let mut iters = VecDeque::new();
        for file in &args.files {
            let file = File::open(file)?;
            iters.push_back(Self::new_iterable(file));
        }
        if iters.is_empty() {
            let file = io::stdin();
            iters.push_back(Self::new_iterable(file));
        }
        Ok(Self(iters))
    }
    fn new_iterable<T: Read + 'static>(file: T) -> Iterable {
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Box::new(lines) as Iterable
    }
}

impl Iterator for BetterIterator {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.get_mut(0) {
                Some(iter) => {
                    if let Some(res) = iter.next() {
                        return Some(res);
                    }
                    self.0.pop_front();
                    continue;
                }
                None => break,
            }
        }
        None
    }
}
