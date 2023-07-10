use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("example.txt");
    run("input.txt");
    badges("input.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let mut result: u32 = 0;
    for line in read.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len() / 2);
        let first = Compartment::new(first);
        let second = Compartment::new(second);
        let common = first.intersect(&second);
        result += common.priority();
    }
    println!("Result for {filename}: {result}");
}

fn badges(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    let lines = read.lines().collect::<Vec<_>>();
    let mut result: u32 = 0;
    lines.chunks(3).for_each(|chunk| {
        result += chunk
            .iter()
            .flatten()
            .map(|x| Compartment::new(x))
            .reduce(|first, second| first.intersect(&second))
            .unwrap()
            .priority();
    });
    println!("Badge priority result: {result}");
}

#[derive(Debug)]
struct Compartment {
    items: HashSet<Item>,
}

impl Compartment {
    fn new(input: &str) -> Self {
        let mut items = HashSet::new();
        input.chars().for_each(|c| {
            let item = Item(c);
            items.insert(item);
        });
        Compartment { items }
    }
    fn intersect(&self, other: &Compartment) -> Compartment {
        let mut res = Compartment::new("");
        self.items.iter().for_each(|k| {
            if other.items.contains(k) {
                res.items.insert(*k);
            }
        });
        res
    }
    fn priority(&self) -> u32 {
        self.items.iter().map(Item::priority).sum::<u32>()
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        let av = self.0 as u32;
        if av <= 96 {
            av - 38
        } else {
            av - 96
        }
    }
}

#[test]
fn test_item_priority() {
    assert_eq!(Item('a').priority(), 1);
    assert_eq!(Item('z').priority(), 26);
    assert_eq!(Item('A').priority(), 27);
    assert_eq!(Item('Z').priority(), 52);
}
