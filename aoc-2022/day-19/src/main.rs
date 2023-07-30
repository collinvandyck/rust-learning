#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let config = Config::parse();
    let blueprints = BufReader::new(File::open(config.filename).unwrap())
        .lines()
        .flatten()
        .map(|l| Blueprint::parse(&l))
        .take(1)
        .collect::<Vec<_>>();
    let factory = Factory { blueprints };
    factory.solve();
}

struct Factory {
    blueprints: Vec<Blueprint>,
}

impl Factory {
    fn solve(&self) {
        println!("Solving for {} blueprints.", self.blueprints.len());
        for blueprint in &self.blueprints {
            let mut solver = Solver::new(blueprint);
            solver.solve();
        }
    }
}

/// Solver finds the optimal solution for one particular blueprint.
struct Solver<'a> {
    blueprint: &'a Blueprint,
}

impl<'a> Solver<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self { blueprint }
    }
    fn solve(&mut self) {
        let state = State::new(self.blueprint);
        println!("Solving for {}", self.blueprint);
        println!("State: {state}");
    }
}

struct State<'a> {
    blueprint: &'a Blueprint,
    amounts: HashMap<Resource, u64>,
}

impl<'a> State<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        let mut amounts = HashMap::new();
        Resource::iter().for_each(|r| {
            amounts.insert(r, 0);
        });
        Self { blueprint, amounts }
    }
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys = self.amounts.keys().sorted().collect::<Vec<_>>();
        let amounts = keys
            .into_iter()
            .map(|k| format!("{}={}", k, self.amounts[k]))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{amounts}")
    }
}

#[derive(Parser)]
struct Config {
    #[arg(short, default_value = "example.txt")]
    filename: String,
}

#[derive(Debug)]
struct Blueprint {
    idx: i32,
    robots: Vec<Robot>,
}

lazy_static! {
    //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    static ref RE: Regex = Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let robots = self
            .robots
            .iter()
            .map(|r| format!("{r}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Blueprint{{[{}] robots=[{}]}}", self.idx, robots)
    }
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let caps = RE.captures(line).unwrap();
        let idx = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let robots = vec![
            Robot {
                resource: Resource::Ore,
                costs: vec![Cost {
                    resource: Resource::Ore,
                    amount: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                }],
            },
            Robot {
                resource: Resource::Clay,
                costs: vec![Cost {
                    resource: Resource::Ore,
                    amount: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                }],
            },
            Robot {
                resource: Resource::Obsidian,
                costs: vec![
                    Cost {
                        resource: Resource::Ore,
                        amount: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                    Cost {
                        resource: Resource::Clay,
                        amount: caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                ],
            },
            Robot {
                resource: Resource::Geode,
                costs: vec![
                    Cost {
                        resource: Resource::Ore,
                        amount: caps.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                    Cost {
                        resource: Resource::Obsidian,
                        amount: caps.get(7).unwrap().as_str().parse::<i32>().unwrap(),
                    },
                ],
            },
        ];
        Blueprint { idx, robots }
    }
}

#[derive(Debug)]
struct Robot {
    costs: Vec<Cost>,
    resource: Resource,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let costs = self
            .costs
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{{r:{} c:[{costs}]}}", self.resource)
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
struct Cost {
    resource: Resource,
    amount: i32,
}

impl Display for Cost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.resource)
    }
}
