use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::prelude::*;

pub struct Input {
    readers: Vec<Box<dyn BufRead>>,
}

impl Input {
    pub fn new(args: &Args) -> CatResult<Self> {
        // build args.files into a vec of buf readers
        let mut readers = vec![];
        for name in &args.files {
            let file = File::open(name)?;
            let reader = BufReader::new(file);
            readers.push(Box::new(reader) as Box<dyn BufRead>);
        }
        if readers.len() == 0 {
            let file = io::stdin();
            let reader = BufReader::new(file);
            readers.push(Box::new(reader) as Box<dyn BufRead>);
        }
        Ok(Self { readers })
    }
}
