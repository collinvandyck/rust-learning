use std::collections::HashMap;

use regex::Regex;

fn main() {
    let example = include_str!("example.txt");
    println!("p1ex={}", count_steps(example));
}

fn count_steps(input: &str) -> usize {
    let src = Node(String::from("AAA"));
    let dst = Node(String::from("ZZZ"));
    parse(input).count_steps(src, dst)
}

struct Map {
    dirs: Vec<Direction>,
    paths: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn count_steps(&self, src: Node, dst: Node) -> usize {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node(String);

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars().map(Direction::from).collect();
    lines.next().unwrap();
    let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
    let paths = lines
        .flat_map(|line| re.captures_iter(line).map(|c| c.extract()).next())
        .map(|(_, [src, left, right])| (src, (left, right)))
        .map(|(s, (l, r))| {
            (
                Node(s.to_string()),
                (Node(l.to_string()), Node(r.to_string())),
            )
        })
        .collect();
    Map { dirs, paths }
}
