#![allow(dead_code, unused)]
use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = (bags_that_can_contain(
        "example.txt",
        Bag::from(("shiny", "gold")),
    )?,);
    println!("p1={p1:?}");
    Ok(())
}

fn bags_that_can_contain(p: impl AsRef<Path>, bag: Bag) -> Result<usize> {
    let rules = build_rules(p)?;
    let bags = rules.bags_that_can_contain(&bag);
    Ok(bags.len())
}

fn build_rules(p: impl AsRef<Path>) -> Result<Rules> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let rules = file_to_lines(p)?
        .into_iter()
        .map(|s| s.parse::<Rule>())
        .collect::<StdResult<Vec<_>, _>>()?;
    Ok(Rules::new(rules))
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Bag {
    shade: Shade,
    hue: Hue,
}

impl Debug for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {}", self.shade.0, self.hue.0))
    }
}

trait IntoBag {
    fn bag(&self) -> Bag;
}

impl IntoBag for &'static str {
    fn bag(&self) -> Bag {
        let parts = self.split(" ").collect::<Vec<_>>();
        Bag::from((parts[0], parts[1]))
    }
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

impl<B> From<B> for Bag
where
    B: IntoBag,
{
    fn from(value: B) -> Self {
        value.bag()
    }
}

#[derive(Default)]
struct Rules {
    rules: Vec<Rule>,
}

impl std::ops::Deref for Rules {
    type Target = Vec<Rule>;
    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl Rules {
    fn new<I>(rules: I) -> Self
    where
        I: IntoIterator<Item = Rule>,
    {
        let mut res = Self::default();
        for rule in rules.into_iter() {
            res.rules.push(rule);
        }
        res
    }

    fn bags_that_can_contain(&self, bag: &Bag) -> Vec<Bag> {
        let mut lookup: HashMap<Bag, HashSet<Bag>> = HashMap::default();
        for rule in &self.rules {
            let mut entry = lookup
                .entry(rule.bag.clone())
                .or_insert_with(|| HashSet::default());
            for (_num, child) in &rule.contains {
                entry.insert(child.clone());
            }
        }
        lookup = dbg!(lookup);
        todo!()
    }
}

struct Rule {
    bag: Bag,
    contains: Vec<(usize, Bag)>,
}

impl Rule {
    fn new(bag: impl Into<Bag>) -> Self {
        Self {
            bag: bag.into(),
            contains: vec![],
        }
    }
    fn contains(mut self, amt: usize, bag: impl Into<Bag>) -> Self {
        self.contains.push((amt, bag.into()));
        self
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

    fn test_bags_can_contain() {
        let bag: Bag = Bag::from("light red");
        let mut rules: Rules = Rules::new([Rule::new("light red")
            .contains(2, "muted yellow")
            .contains(1, "bright white")]);
    }

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
