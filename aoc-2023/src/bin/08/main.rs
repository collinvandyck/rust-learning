use std::collections::{HashMap, HashSet};

use regex::Regex;

fn main() {
    let example = include_str!("example.txt");
    let example2 = include_str!("example-2.txt");
    let input = include_str!("input.txt");
    println!("p1ex1={}", count_steps(example));
    println!("p1ex2={}", count_steps(example2));
    println!("p1in={}", count_steps(input));
    println!("p2ex1={}", ghost_steps(include_str!("example-p2.txt")));
    println!("p2in={}", optimized_ghost_steps(include_str!("input.txt")));
}

fn count_steps(input: &str) -> usize {
    let src = Node(String::from("AAA"));
    let dst = Node(String::from("ZZZ"));
    parse(input).count_steps(src, dst)
}

fn ghost_steps(input: &str) -> usize {
    parse(input).ghost_mode()
}

fn optimized_ghost_steps(input: &str) -> usize {
    parse(input).optimized_ghost_mode()
}

struct Map {
    dirs: Vec<Direction>,
    paths: HashMap<Node, (Node, Node)>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Key<'a>(usize, Direction, &'a Node);

impl Map {
    fn cycle_for(&self, start: &Node) -> Vec<usize> {
        let mut node = start;
        let mut cycles = vec![];
        let mut set: HashSet<(usize, &Node)> = HashSet::default();
        let dirs = self.dirs.iter().copied().enumerate().cycle();
        for (idx, (di, dir)) in dirs.enumerate() {
            if node.is_end() {
                let key = (di, node);
                if set.contains(&key) {
                    break;
                }
                println!("Found {start:?} -> ({idx}/{di} {node:?})");
                set.insert(key);
                cycles.push(idx);
            }
            let (left, right) = self.paths.get(node).unwrap();
            if dir == Direction::Left {
                node = left;
            } else {
                node = right;
            }
        }
        cycles
    }

    fn optimized_ghost_mode(&self) -> usize {
        let mut nodes: Vec<_> = self.paths.keys().filter(|n| n.is_start()).collect();
        nodes.sort();
        let mut cycles: Vec<(&Node, Vec<usize>)> = nodes
            .into_iter()
            .map(|n| (n, self.cycle_for(n)))
            .map(|(node, cycle)| {
                println!("node {node:?} cycle: {cycle:?}");
                (node, cycle)
            })
            .collect();
        println!("Finding LCM");
        let idxs: Vec<usize> = cycles
            .iter()
            .flat_map(|(_, idxs)| idxs.iter())
            .copied()
            .collect();
        let mut product: usize = idxs.iter().product();
        println!("Product: {product}");
        // for each index, test to see if when it divides the product that the result is
        // divisible by the other indexes. if it is, update the product.
        //
        // 4 8 12
        // product is 384
        // test 4: = 96
        //  96 % 8 = 0
        //test 12
        //  96 % 12 = 0
        // product is now 96
        // test 4: = 24
        //  8 passes
        //  12 passes
        //product is now 24
        // test 4: = 6
        //  fails on 8
        // test 8: 3
        //  fails on 4
        // test 12: = 2
        //  fails on 4
        //
        //
        // node Node("AAA") cycle: [13301]
        // node Node("BGA") cycle: [18961]
        // node Node("JNA") cycle: [16697]
        // node Node("PTA") cycle: [17263]
        // node Node("SLA") cycle: [12169]
        // node Node("XJA") cycle: [14999]
        // Finding LCM
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        // Product: 10456399072945950237
        loop {
            println!("Product: {product}");
            for idx in idxs.clone() {
                if idx == product {
                    println!("Product found to match one of the idx's ({idx})");
                    return product;
                }
                let div = product / idx;
                // see if all of the idx's divide into (product/idx) without remainder
                if idxs.iter().all(|i| div % *i == 0) {
                    product = div;
                }
            }
        }
    }

    fn ghost_mode(&self) -> usize {
        let dirs = self.dirs.iter().copied().cycle();
        let mut count = 0;
        let mut starts = self
            .paths
            .keys()
            .filter(|n| n.0.ends_with('A'))
            .collect::<Vec<_>>();
        for dir in dirs {
            if starts.iter().all(|n| n.0.ends_with('Z')) {
                break;
            }
            for node in starts.iter_mut() {
                let (left, right) = self.paths.get(node).unwrap();
                if dir == Direction::Left {
                    *node = left;
                } else {
                    *node = right;
                }
            }
            count += 1;
        }
        count
    }
    fn count_steps(&self, src: Node, dst: Node) -> usize {
        let mut count = 0;
        let dirs = self.dirs.iter().copied().cycle();
        let mut cur = &src;
        for dir in dirs {
            if cur == &dst {
                break;
            }
            cur = match (dir, self.paths.get(&cur).unwrap()) {
                (Direction::Left, (left, _)) => left,
                (Direction::Right, (_, right)) => right,
            };
            count += 1;
        }
        count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node(String);

impl Node {
    fn is_start(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_end(&self) -> bool {
        self.0.ends_with('Z')
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
