use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    run("example.txt");
}

fn run(filename: &str) {
    let file = File::open(filename).unwrap();
    let read = BufReader::new(file);
    for line in read.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len() / 2);
        let first = Compartment::new(first);
        let second = Compartment::new(second);
    }
}

struct Compartment {
    items: HashMap<Item, u32>,
}

impl Compartment {
    fn new(input: &str) -> Self {
        let mut items = HashMap::new();
        input.chars().for_each(|c| {
            let item = Item(c);
            let entry = items.entry(item).or_insert(0);
            *entry += 1;
        });
        Compartment { items }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        let mut av = self.0 as u32;
        dbg!((self.0, av));
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
