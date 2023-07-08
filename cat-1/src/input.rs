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
        let current = None;
        Ok(Self { readers, current })
    }
}

impl Iterator for Input {
    type Item = CatResult<String>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() && self.readers.is_empty() {
            return None;
        }
        if self.current.is_none() {
            self.current = self.readers.pop_front();
        }
        match self.current {
            None => None,
            Some(ref mut reader) => {
                let mut buf = String::new();
                let read = reader.read_line(&mut buf);
                let done = match read {
                    Err(_) => true,
                    Ok(0) => true,
                    _ => false,
                };
                if done {
                    self.current = None;
                    return None;
                }
                let buf = buf.trim_end().to_string();
                Some(Ok(buf))
            }
        }
    }
}
