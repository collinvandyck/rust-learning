#![allow(dead_code, unused)]
use std::fmt::Display;

use aoc_2020::prelude::*;
use tracing::{debug, info};
use tracing_subscriber::field::debug;

fn main() -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Bag(String);

impl Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    bag: Bag,
    counts: HashMap<Bag, usize>,
}

impl Rule {
    fn new(bag: Bag) -> Self {
        Self {
            bag,
            counts: HashMap::default(),
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+ \w+) bags contain (.*)$").unwrap());
        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("first regex failed"))?;
        let bag = Bag(caps.get(1).unwrap().as_str().to_string());
        let res = caps.get(2).unwrap().as_str();
        let counts = if res != "no other bags." {
            res.split(", ")
                .map(|p| p.split(" ").take(3).collect::<Vec<_>>())
                .map(|p| {
                    let num = p[0].parse::<usize>().unwrap();
                    let bag = Bag(format!("{} {}", p[1], p[2]));
                    (bag, num)
                })
                .collect::<HashMap<Bag, usize>>()
        } else {
            HashMap::default()
        };
        Ok(Rule { bag, counts })
    }
}

fn topo_sort(mut rules: Vec<Rule>) -> Vec<Bag> {
    let mut roots: Vec<Rule> = rules
        .iter()
        .filter(|r| r.counts.is_empty())
        .cloned()
        .collect();
    let mut rules: Vec<Rule> = rules.into_iter().filter(|r| !roots.contains(r)).collect();
    let mut sorted: Vec<Bag> = vec![];
    while let Some(rule) = roots.pop() {
        sorted.push(rule.bag.clone());
        // remove rule counts where they reference `rule`.
        rules.iter_mut().for_each(|rr| {
            rr.counts.remove_entry(&rule.bag);
            if rr.counts.is_empty() {
                roots.push(rr.clone());
            }
        });
        rules.retain(|rr| !rr.counts.is_empty());
    }
    if !rules.is_empty() {
        panic!("Impossible topological sort");
    }
    sorted
}

fn build_rules(p: impl AsRef<Path>) -> Result<Vec<Rule>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    file_to_lines(p)?
        .into_iter()
        .map(|l| l.parse::<Rule>())
        .collect::<StdResult<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_rule_topo_sort() {
        let rules = build_rules("example.txt").unwrap();
        let bags = topo_sort(rules);
        let names = bags.iter().map(|b| &b.0).collect::<Vec<_>>();
        assert_eq!(
            names,
            &[
                "dotted black",
                "faded blue",
                "vibrant plum",
                "dark olive",
                "shiny gold",
                "muted yellow",
                "bright white",
                "dark orange",
                "light red",
            ]
        );
        let rules = build_rules("input.txt").unwrap();
        let bags = topo_sort(rules.clone());
        assert_eq!(bags.len(), rules.len());
    }

    #[test]
    fn test_rule_parse() {
        let rules = build_rules("example.txt").unwrap();
        assert_eq!(rules.len(), 9);
        let rules = build_rules("input.txt").unwrap();
        assert_eq!(rules.len(), 594);
    }
}
