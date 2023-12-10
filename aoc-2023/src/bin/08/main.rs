use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    println!("p1ex={}", count_steps(example));
    println!("p1in={}", count_steps(input));
}

fn count_steps(input: &str) -> usize {
    let map = parse(input);
    let node: Node = "AAA".into();
    let mut node = &node;
    let mut count = 0;
    for dir in map.dirs() {
        if node.0 == "ZZZ" {
            break;
        }
        node = map.next(node, dir);
        count += 1;
    }
    count
}

struct Map {
    dirs: Vec<Dir>,
    paths: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn dirs(&self) -> impl Iterator<Item = Dir> + '_ {
        self.dirs.iter().copied().cycle()
    }
    fn next(&self, n: &Node, dir: Dir) -> &Node {
        if let Some((left, right)) = self.paths.get(n) {
            if dir == Dir::Left {
                return left;
            } else {
                return right;
            }
        }
        panic!("no path for {:?}", n);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node(String);

impl From<&'static str> for Node {
    fn from(value: &'static str) -> Self {
        Node(value.to_string())
    }
}

impl Node {
    fn is_start(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_end(&self) -> bool {
        self.0.ends_with('Z')
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'R' => Dir::Right,
            'L' => Dir::Left,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Map {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars().map(Dir::from).collect();
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
