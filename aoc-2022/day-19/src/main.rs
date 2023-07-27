use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let config = Config::parse();
    let lines = BufReader::new(File::open(&config.filename).unwrap()).lines();
    for line in lines.flatten() {
        let blueprint = Blueprint::parse(&line);
        println!("{blueprint:?}");
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
impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let caps = RE.captures(line).unwrap();
        let idx = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let mut robots = vec![];
        robots.push(Robot {
            resource: Resource::Ore,
            costs: vec![Cost {
                resource: Resource::Ore,
                amount: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            }],
        });
        robots.push(Robot {
            resource: Resource::Clay,
            costs: vec![Cost {
                resource: Resource::Ore,
                amount: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            }],
        });
        robots.push(Robot {
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
        });
        robots.push(Robot {
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
        });
        Blueprint { idx, robots }
    }
}

#[derive(Debug)]
struct Robot {
    costs: Vec<Cost>,
    resource: Resource,
}

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Cost {
    resource: Resource,
    amount: i32,
}
