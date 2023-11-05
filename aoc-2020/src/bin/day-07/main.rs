#![allow(dead_code, unused)]
use aoc_2020::prelude::*;

fn main() -> Result<()> {
    Ok(())
}

fn process(p: impl AsRef<Path>) -> Result<()> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let lines = file_to_lines(p)?;
    Ok(())
}

fn build_rules(p: impl AsRef<Path>) -> Result<Vec<Rule>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    Ok(file_to_lines(p)?
        .into_iter()
        .map(|s| s.parse::<Rule>())
        .collect::<StdResult<Vec<_>, _>>()?)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Hue(String);

impl<V> From<V> for Hue
where
    V: AsRef<str>,
{
    fn from(value: V) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl std::ops::Deref for Hue {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Shade(String);

impl<V> From<V> for Shade
where
    V: AsRef<str>,
{
    fn from(value: V) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl std::ops::Deref for Shade {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag {
    shade: Shade,
    hue: Hue,
}

impl<S, H> From<(S, H)> for Bag
where
    S: Into<Shade>,
    H: Into<Hue>,
{
    fn from((shade, hue): (S, H)) -> Self {
        Self {
            shade: shade.into(),
            hue: hue.into(),
        }
    }
}

struct Rule {
    bag: Bag,
    contains: Vec<(usize, Bag)>,
}

impl Rule {
    fn new(color: Bag) -> Self {
        Self {
            bag: color,
            contains: vec![],
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        static RE1: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+) (\w+) bags contain (.*)$").unwrap());
        let caps = RE1
            .captures(s)
            .ok_or_else(|| anyhow!("first regex failed"))?;
        let shade = Shade(caps.get(1).unwrap().as_str().to_string());
        let hue = Hue(caps.get(2).unwrap().as_str().to_string());
        let bag = Bag { shade, hue };
        let mut rule = Rule::new(bag);
        let res = caps.get(3).unwrap().as_str();
        if res != "no other bags." {
            for (num, color) in res
                .split(", ")
                .map(|p| p.split(" ").take(3).collect::<Vec<_>>())
                .map(|p| {
                    let num = p[0].parse::<usize>().unwrap();
                    let shade = Shade(p[1].to_string());
                    let hue = Hue(p[2].to_string());
                    let color = Bag { shade, hue };
                    (num, color)
                })
            {
                rule.contains.push((num, color));
            }
        }
        Ok(rule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let rules = build_rules("input.txt").unwrap();
        assert_eq!(rules.len(), 594);
        let rules = build_rules("example.txt").unwrap();
        assert_eq!(rules.len(), 9);
        let rule = rules
            .iter()
            .find(|r| r.bag.shade.as_str() == "faded" && r.bag.hue.as_str() == "blue");
        assert!(rule.is_some());
        assert_eq!(rule.map(|r| r.contains.len()), Some(0));
        let rule = rules
            .iter()
            .find(|r| r.bag.shade.as_str() == "vibrant" && r.bag.hue.as_str() == "plum");
        assert!(rule.is_some());
        assert_eq!(rule.map(|r| r.contains.len()), Some(2));
        assert_eq!(
            rule.map(|r| r.contains[0].clone()),
            Some((5, Bag::from(("faded", "blue"))))
        );
        assert_eq!(
            rule.map(|r| r.contains[1].clone()),
            Some((6, Bag::from(("dotted", "black"))))
        );
    }
}
