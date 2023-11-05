use aoc_2020::prelude::*;

fn main() -> Result<()> {
    build_groups("example.txt")?;
    Ok(())
}

fn build_groups(p: impl AsRef<Path>) -> Result<Vec<Group>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let groups = file_to_lines(p)?
        .grouped(|l| l.is_empty())
        .into_iter()
        .map(|v| v.join(""))
        .map(|s| s.chars().collect::<Vec<_>>())
        .map(Group::from_chars)
        .collect::<Vec<_>>();
    for group in groups {
        println!("group: {group:?}");
    }
    todo!()
}

#[derive(Debug)]
struct Group(HashSet<char>);

impl Group {
    fn from_chars<I>(chars: I) -> Self
    where
        I: IntoIterator<Item = char>,
    {
        let mut set = HashSet::default();
        for ch in chars {
            set.insert(ch);
        }
        Self(set)
    }
}
