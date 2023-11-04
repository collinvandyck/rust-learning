use std::str::FromStr;

use aoc_2020::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let p1 = (
        num_valid_pws("example.txt", Policy::Old)?,
        num_valid_pws("input.txt", Policy::Old)?,
    );
    println!("p1={p1:?}");
    let p2 = (
        num_valid_pws("example.txt", Policy::New)?,
        num_valid_pws("input.txt", Policy::New)?,
    );
    println!("p2={p2:?}");
    Ok(())
}

fn num_valid_pws(p: impl AsRef<Path>, policy: Policy) -> Result<usize> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let num = file_to_lines(p)?
        .into_iter()
        .map(|s| s.parse::<Entry>())
        .collect::<StdResult<Vec<_>, _>>()?
        .into_iter()
        .filter(|p| p.valid(policy))
        .count();
    Ok(num)
}

struct Entry {
    min: usize,
    max: usize,
    ch: char,
    pass: String,
}

#[derive(Clone, Copy, Debug)]
enum Policy {
    Old,
    New,
}

impl Entry {
    fn valid(&self, policy: Policy) -> bool {
        match policy {
            Policy::Old => {
                let count = self.pass.chars().filter(|c| c == &self.ch).count();
                count >= self.min && count <= self.max
            }
            Policy::New => {
                let chars: Vec<_> = self.pass.chars().collect();
                let one = chars.len() > (self.min - 1) && chars[self.min - 1] == self.ch;
                let two = chars.len() > (self.max - 1) && chars[self.max - 1] == self.ch;
                (one != two) && (one || two)
            }
        }
    }
}

impl FromStr for Entry {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| regex::Regex::from_str(r"(\d+)-(\d+) (.): (.*)$").unwrap());
        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!(format!("pattern match fail for '{s}'")))?;
        let min = caps.get(1).unwrap().as_str().parse::<usize>()?;
        let max = caps.get(2).unwrap().as_str().parse::<usize>()?;
        let ch = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
        let pass = caps.get(4).unwrap().as_str().to_string();
        Ok(Self { min, max, ch, pass })
    }
}
