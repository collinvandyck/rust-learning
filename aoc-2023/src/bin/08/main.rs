use num::Integer;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let example = include_str!("example.txt");
    let input = include_str!("input.txt");
    let ex2 = include_str!("example-p2.txt");
    println!("p1ex={}", min_distance(example, true));
    println!("p1in={}", min_distance(input, true));
    println!("p2ex={}", min_distance(ex2, false));
    println!("p2in={}", min_distance(input, false));
}

fn min_distance(input: &str, pt1: bool) -> usize {
    let map = parse(input);
    map.starts()
        .iter()
        .filter(|n| !pt1 || n.0 == "AAA")
        .map(|n| get_cycle(n, &map))
        .map(|c| c.finishes[0])
        .reduce(|a, b| a.lcm(&b))
        .unwrap_or_default()
}

#[test]
fn test_part_1() {
    assert_eq!(min_distance(include_str!("example.txt"), true), 2);
    assert_eq!(min_distance(include_str!("input.txt"), true), 13301);
}

#[test]
fn test_part_2() {
    assert_eq!(min_distance(include_str!("example-p2.txt"), false), 6);
    assert_eq!(
        min_distance(include_str!("input.txt"), false),
        7309459565207
    );
}

fn get_cycle(start: &Node, map: &Map) -> Cycle {
    let mut node = start;
    let mut cycle = Cycle::default();
    let mut lookup: HashSet<(&Node, IdDir)> = HashSet::default();
    for (count, id_dir) in map.id_dirs().enumerate() {
        let key = (node, id_dir);
        if lookup.contains(&key) {
            break;
        }
        lookup.insert(key);
        cycle.length = count;
        if node.is_end() {
            cycle.finishes.push(count);
        }
        node = map.next(node, id_dir.dir);
    }
    cycle
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Cycle {
    length: usize,
    finishes: Vec<usize>,
}

struct Map {
    dirs: Vec<Dir>,
    paths: HashMap<Node, (Node, Node)>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct IdDir {
    id: usize,
    dir: Dir,
}

impl Map {
    fn id_dirs(&self) -> impl Iterator<Item = IdDir> + '_ {
        self.dirs
            .iter()
            .copied()
            .enumerate()
            .map(|(id, dir)| IdDir { id, dir })
            .cycle()
    }
    fn starts(&self) -> Vec<&Node> {
        self.paths.keys().filter(|n| n.is_start()).collect()
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
