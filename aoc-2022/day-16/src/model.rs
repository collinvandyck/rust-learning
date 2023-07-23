use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Name([u8; 2]);

impl From<&[u8]> for Name {
    fn from(value: &[u8]) -> Self {
        Self([value[0], value[1]])
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Name::from(value.as_bytes())
    }
}

impl Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
    }
}

#[derive(Debug, Clone)]
pub struct Valve {
    pub name: Name,
    pub rate: u64,
    pub links: Vec<Name>,
}

impl Valve {
    fn new(name: Name, rate: u64, links: Vec<Name>) -> Self {
        Self { name, rate, links }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{:2}] -> {:?}", self.name, self.rate, self.links)
    }
}

pub struct Parser {
    re: Regex,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            //Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            re: Regex::new(r#"Valve (.*).*rate=(.*);.*leads? to valves?(.*)"#).unwrap(),
        }
    }
    pub fn read_file(filename: &str) -> Vec<Valve> {
        let parser = Self::new();
        let file = File::open(filename).unwrap();
        let read = BufReader::new(file);
        read.lines()
            .map(Result::unwrap)
            .map(|l| parser.valve(&l))
            .collect::<Vec<_>>()
    }
    pub fn valve(&self, s: &str) -> Valve {
        let caps = self.re.captures(s).unwrap();
        let (_, [name, rate, links]) = caps.extract();
        let name: Name = name.as_bytes().into();
        let rate = rate.parse::<u64>().unwrap();
        let links: Vec<Name> = links
            .trim()
            .split(',')
            .map(|s| s.trim().as_bytes().into())
            .collect::<Vec<_>>();
        Valve::new(name, rate, links)
    }
}
