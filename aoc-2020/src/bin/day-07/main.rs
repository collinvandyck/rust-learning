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
    let lines = file_to_lines(p)?;
    Ok(vec![])
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
