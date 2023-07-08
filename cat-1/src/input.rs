use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::prelude::*;

pub struct Input {
    readers: VecDeque<Box<dyn BufRead>>,
    current: Option<Box<dyn BufRead>>,
}

impl Input {
    pub fn new(args: &Args) -> CatResult<Self> {
        // build args.files into a vec of buf readers
        let mut readers = VecDeque::new();
        for name in &args.files {
            let file = File::open(name)?;
            let reader = BufReader::new(file);
            readers.push_back(Box::new(reader) as Box<dyn BufRead>);
        }
        if readers.len() == 0 {
            let file = io::stdin();
            let reader = BufReader::new(file);
            readers.push_back(Box::new(reader) as Box<dyn BufRead>);
        }
        let current = readers.pop_front();
        Ok(Self { readers, current })
    }
}

impl Iterator for Input {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some("foo".to_string())
    }
}
