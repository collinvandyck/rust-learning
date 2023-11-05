use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = ["example.txt", "input.txt"]
        .into_iter()
        .map(|f| count_uniq_answers(f, Mode::Any))
        .collect::<StdResult<Vec<_>, _>>()?;
    println!("p1={p1:?}");
    let p2 = ["example.txt", "input.txt"]
        .into_iter()
        .map(|f| count_uniq_answers(f, Mode::All))
        .collect::<StdResult<Vec<_>, _>>()?;
    println!("p22{p2:?}");
    Ok(())
}

fn count_uniq_answers(p: impl AsRef<Path>, mode: Mode) -> Result<usize> {
    Ok(build_groups(p, mode)?
        .into_iter()
        .map(|g| g.uniq_answers())
        .sum::<usize>())
}

fn build_groups(p: impl AsRef<Path>, mode: Mode) -> Result<Vec<Group>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let groups = file_to_lines(p)?
        .grouped(|l| l.is_empty())
        .into_iter()
        .map(|strings: Vec<String>| {
            let mut group = Group::new(mode);
            for s in strings {
                group.push(s.chars());
            }
            group
        })
        .collect::<Vec<_>>();
    Ok(groups)
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Any,
    All,
}

#[derive(Debug)]
struct Group {
    mode: Mode,
    votes: Vec<HashSet<char>>,
}

impl Group {
    fn new(mode: Mode) -> Self {
        Self {
            mode,
            votes: vec![],
        }
    }

    fn push<I>(&mut self, chars: I) -> &mut Self
    where
        I: IntoIterator<Item = char>,
    {
        self.votes.push(chars.into_iter().collect());
        self
    }

    fn uniq_answers(&self) -> usize {
        match self.mode {
            Mode::Any => self
                .votes
                .clone()
                .into_iter()
                .reduce(|acc, h| acc.union(&h).copied().collect::<HashSet<_>>())
                .map(|v| v.len())
                .unwrap_or_default(),
            Mode::All => self
                .votes
                .clone()
                .into_iter()
                .reduce(|acc, h| acc.intersection(&h).copied().collect::<HashSet<_>>())
                .map(|v| v.len())
                .unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_all() {
        let mut g1 = Group::new(Mode::All);
        g1.push("abc".chars()).push("bc".chars()).push("ac".chars());
        assert_eq!(g1.uniq_answers(), 1);
    }
}
