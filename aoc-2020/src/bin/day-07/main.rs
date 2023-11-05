#![allow(dead_code, unused)]
use aoc_2020::prelude::*;
use tracing::debug;
use tracing_subscriber::field::debug;

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
    let bags = rules.bags_that_can_contain(bag.clone());
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

    /// Returns the number of bags that the specified bag can contain
    fn inflate_bag(&self, bag: impl Into<Bag>) -> HashMap<Bag, usize> {
        let mut queue: Vec<Bag> = vec![bag.into()];
        let mut res = HashMap::default();
        while let Some(bag) = queue.pop() {
            if let Some(rule) = self.rules.iter().find(|r| r.bag == bag) {
                let rule = self.rule_for(&bag);
                let Rule { counts, .. } = self.rule_for(&bag);
                for (amount, bag) in counts {
                    *res.entry(bag.clone()).or_insert(0) += amount;
                    queue.push(bag.clone());
                }
            }
        }
        res
    }

    fn rule_for(&self, b: &Bag) -> &Rule {
        match self.rules.iter().find(|rule| &rule.bag == b) {
            Some(rule) => rule,
            None => panic!("no rule found for {b:?}"),
        }
    }

    fn bags_that_can_contain(&self, bag: impl Into<Bag>) -> Vec<Bag> {
        todo!()
    }
}

#[derive(Debug)]
struct Rule {
    bag: Bag,
    counts: Vec<(usize, Bag)>,
}

impl Rule {
    fn new(bag: impl Into<Bag>) -> Self {
        Self {
            bag: bag.into(),
            counts: vec![],
        }
    }
    fn contains(mut self, amt: usize, bag: impl Into<Bag>) -> Self {
        self.counts.push((amt, bag.into()));
        self
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+) (\w+) bags contain (.*)$").unwrap());
        let caps = RE
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
                rule.counts.push((num, color));
            }
        }
        Ok(rule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_inflate_bag() {
        let mut rules = Rules::new([
            Rule::new("light red")
                .contains(2, "muted yellow")
                .contains(1, "bright white"),
            Rule::new("dark orange")
                .contains(3, "bright white")
                .contains(4, "muted yellow"),
            Rule::new("bright white").contains(1, "shiny gold"),
        ]);
        assert_eq!(
            rules.inflate_bag("light red"),
            HashMap::from([
                (Bag::from("muted yellow"), 2),
                (Bag::from("bright white"), 1),
                (Bag::from("shiny gold"), 1),
            ]),
        );
        assert_eq!(
            rules.inflate_bag("dark orange"),
            HashMap::from([
                (Bag::from("muted yellow"), 4),
                (Bag::from("bright white"), 3),
                (Bag::from("shiny gold"), 1),
            ]),
        );
    }

    #[test]
    #[traced_test]
    #[ignore]
    fn test_bags_can_contain() {
        let mut rules = Rules::new([
            Rule::new("light red")
                .contains(2, "muted yellow")
                .contains(1, "bright white"),
            Rule::new("dark orange")
                .contains(3, "bright white")
                .contains(4, "muted yellow"),
            Rule::new("bright white").contains(1, "shiny gold"),
        ]);
        assert_eq!(
            rules.bags_that_can_contain("shiny gold"),
            bags(["bright white"])
        );
    }

    fn bags<I, T>(items: I) -> Vec<Bag>
    where
        I: IntoIterator<Item = T>,
        T: Into<Bag>,
    {
        items.into_iter().map(|f| f.into()).collect::<Vec<_>>()
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
        assert_eq!(rule.map(|r| r.counts.len()), Some(0));
        let rule = rules
            .iter()
            .find(|r| r.bag.shade.as_str() == "vibrant" && r.bag.hue.as_str() == "plum");
        assert!(rule.is_some());
        assert_eq!(rule.map(|r| r.counts.len()), Some(2));
        assert_eq!(
            rule.map(|r| r.counts[0].clone()),
            Some((5, Bag::from(("faded", "blue"))))
        );
        assert_eq!(
            rule.map(|r| r.counts[1].clone()),
            Some((6, Bag::from(("dotted", "black"))))
        );
    }
}
