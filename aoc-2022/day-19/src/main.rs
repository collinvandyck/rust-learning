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
    vec,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const MAX_TURNS: usize = 24;

lazy_static! {
    //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    static ref RE: Regex = Regex::new(r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();
}

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
            Self::solve_blueprint(blueprint);
        }
    }
    fn solve_blueprint(blueprint: &Blueprint) -> State {
        println!("Solving for {}\n", blueprint);
        let state = State::new(blueprint);
        let state = Self::solve_state(state);
        println!("Solution (score={}):\n{state}", state.score());
        state
    }
    fn solve_state(state: State) -> State {
        if state.is_done() {
            return state;
        }
        let mut solution: Option<State> = None;
        let nexts = state.next_states();
        for next in nexts {
            let this = Self::solve_state(next);
            solution = match solution {
                Some(prev) if this.score() > prev.score() => Some(this),
                Some(prev) => Some(prev),
                None => Some(this),
            }
        }
        solution.unwrap()
    }
}

#[derive(Clone)]
struct State<'a> {
    blueprint: &'a Blueprint,
    amounts: HashMap<Resource, u64>,
    robots: HashMap<Resource, u64>,
    max_turns: usize,
    turn: usize,
}

impl<'a> State<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        let amounts = Resource::iter().map(|r| (r, 0)).collect();
        // we always start with one ore collecting robot
        let robots = HashMap::from([(Resource::Ore, 1)]);
        let max_turns = MAX_TURNS;
        let turn = 0;
        Self {
            blueprint,
            amounts,
            robots,
            max_turns,
            turn,
        }
    }
    fn next_states(&self) -> Vec<State<'a>> {
        let mut res = vec![];

        // do nothing.
        let mut clone = self.clone();
        clone.mine();
        res.push(clone);

        res
    }

    /// mines resources for each robot. bumps the turns counter.
    fn mine(&mut self) {
        self.robots.iter().for_each(|(res, count)| {
            let entry = self.amounts.entry(*res).or_insert(0);
            *entry += *count;
        });
        self.turn += 1;
    }

    /// are we out of turns?
    fn is_done(&self) -> bool {
        self.turn == self.max_turns
    }

    /// Returns the number of geode resources
    fn score(&self) -> u64 {
        *self.amounts.get(&Resource::Geode).unwrap_or(&0_u64)
    }
}

#[test]
fn test_index_hashmap() {
    let m = HashMap::from([(Resource::Ore, 1)]);
    assert_eq!(m[&Resource::Ore], 1);
    assert_eq!(m.get(&Resource::Geode).unwrap_or(&42), &42);
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amounts = display_map(&self.amounts);
        let robots = display_map(&self.robots);
        write!(
            f,
            "Turn:    {}\nAmounts: {amounts}\nRobots:  {robots}",
            self.turn
        )
    }
}

/// Prints a hashmap's entries. Requires entries be sortable and displayable.
fn display_map<K, V>(m: &HashMap<K, V>) -> String
where
    K: Ord + Display,
    V: Ord + Display,
{
    m.iter()
        .sorted()
        .map(|e| format!("{}={}", e.0, e.1))
        .join(", ")
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

#[derive(EnumIter, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
