#![allow(dead_code, unused)]
use aoc_2020::prelude::*;
use tracing::{debug, info};
use tracing_subscriber::field::debug;

fn main() -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Bag(String);

#[derive(Debug)]
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

fn build_rules(p: impl AsRef<Path>) -> Result<Vec<Rule>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    file_to_lines(p)?
        .into_iter()
        .map(|l| l.parse::<Rule>())
        .collect::<StdResult<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use crate::build_rules;

    #[test]
    fn test_rule_parse() {
        let rules = build_rules("example.txt").unwrap();
        assert_eq!(rules.len(), 9);
    }
}
