use std::default;

use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = ["example.txt", "input.txt"]
        .into_iter()
        .map(|f| count_uniq_answers(f, Mode::Any))
        .collect::<StdResult<Vec<_>, _>>()?;
    println!("p1={p1:?}");
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
    Ok(file_to_lines(p)?
        .grouped(|l| l.is_empty())
        .into_iter()
        .map(|strings: Vec<String>| {
            let mut group = Group::new(mode);
            for s in strings {
                group.push(s.chars());
            }
            group
        })
        .collect::<Vec<_>>())
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

    fn push<I>(&mut self, chars: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.votes.push(chars.into_iter().collect());
    }

    fn uniq_answers(&self) -> usize {
        match self.mode {
            Mode::Any => self
                .votes
                .clone()
                .into_iter()
                .reduce(|mut f, b| {
                    f.extend(b);
                    f
                })
                .map(|v| v.len())
                .unwrap_or_default(),
            Mode::All => todo!(),
        }
    }
}
