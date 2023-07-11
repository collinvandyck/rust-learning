#![warn(clippy::all, clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Line {
    Cd(String),
    Ls(),
    Dir(String),
    File(u64, String),
}

pub fn parse_lines(filename: &str) -> Vec<Line> {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    read.lines()
        .flatten()
        .map(|s| {
            let parts = s.as_str().split(' ').collect::<Vec<&str>>();
            match parts[..] {
                ["$", "cd", dir] => Line::Cd(dir.to_string()),
                ["$", "ls"] => Line::Ls(),
                ["dir", dir] => Line::Dir(dir.to_string()),
                [size, name] => Line::File(size.parse::<u64>().unwrap(), name.to_string()),
                _ => panic!("parse error"),
            }
        })
        .collect::<Vec<_>>()
}
