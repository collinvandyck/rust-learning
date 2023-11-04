use aoc_2020::prelude::*;

fn main() -> Result<()> {
    get_passports("example.txt")?;
    Ok(())
}

fn get_passports(p: impl AsRef<Path>) -> Result<Passport> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let mut lines = file_to_lines(p)?.into_iter();
    loop {
        let line = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if line.is_empty() {
            break;
        }
        let map = line
            .split(" ")
            .map(|s| s.splitn(2, ":").collect::<Vec<_>>())
            .map(|p| (p[0], p[1]))
            .collect::<HashMap<_, _>>();
        println!("Got map: {map:?}");
    }
    todo!()
}

struct Passport {}
