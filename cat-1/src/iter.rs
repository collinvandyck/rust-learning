use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Read, Result},
};

use crate::prelude::*;

type IterType = Box<dyn Iterator<Item = io::Result<String>>>;

pub struct BetterIter {
    iters: VecDeque<IterType>,
}

impl BetterIter {
    pub fn new(args: &Args) -> CatResult<BetterIter> {
        let mut iters = VecDeque::new();
        for file in &args.files {
            let file = File::open(file)?;
            iters.push_back(Self::iter_from_read(file));
        }
        if iters.is_empty() {
            let file = io::stdin();
            iters.push_back(Self::iter_from_read(file));
        }
        Ok(Self { iters })
    }
    fn iter_from_read<T: Read + 'static>(file: T) -> IterType {
        let reader = BufReader::new(file);
        let lines = reader.lines();
        let iter = Box::new(lines) as IterType;
        iter
    }
}

impl Iterator for BetterIter {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.iters.is_empty() {
            match self.iters.get_mut(0) {
                Some(ref mut iter) => match iter.next() {
                    Some(res) => return Some(res),
                    None => {
                        self.iters.pop_front();
                        continue;
                    }
                },
                None => break,
            }
        }
        None
    }
}
