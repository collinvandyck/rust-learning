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

fn rules(p: impl AsRef<Path>) -> Result<Vec<Rule>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    Ok(file_to_lines(p)?
        .into_iter()
        .map(|s| s.parse::<Rule>())
        .collect::<StdResult<Vec<_>, _>>()?)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Hue(String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Shade(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Color {
    shade: Shade,
    hue: Hue,
}

struct Rule {
    color: Color,
    contains: Vec<(usize, Color)>,
}

impl Rule {
    fn new(color: Color) -> Self {
        Self {
            color,
            contains: vec![],
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        static RE1: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^(\w+) (\w+) bags contain (.*)$").unwrap());
        let caps = RE1
            .captures(s)
            .ok_or_else(|| anyhow!("first regex failed"))?;
        let shade = Shade(caps.get(1).unwrap().as_str().to_string());
        let hue = Hue(caps.get(2).unwrap().as_str().to_string());
        let color = Color { shade, hue };
        let mut rule = Rule::new(color);
        let res = caps.get(3).unwrap().as_str();
        if res != "no other bags." {
            for (num, color) in res
                .split(", ")
                .map(|p| p.split(" ").take(3).collect::<Vec<_>>())
                .map(|p| {
                    let num = p[0].parse::<usize>().unwrap();
                    let shade = Shade(p[1].to_string());
                    let hue = Hue(p[2].to_string());
                    let color = Color { shade, hue };
                    (num, color)
                })
            {
                rule.contains.push((num, color));
            }
        }
        Ok(rule)
    }
}

struct Bag {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let rules = rules("example.txt").unwrap();
        assert_eq!(rules.len(), 9);
    }
}
