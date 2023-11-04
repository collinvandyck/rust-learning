use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let passports = get_passports("example.txt")?;
    println!("Got passports: {passports:?}");
    Ok(())
}

fn get_passports(p: impl AsRef<Path>) -> Result<Vec<Passport>> {
    let p = PathBuf::from(file!()).parent().unwrap().join(p.as_ref());
    let mut lines = file_to_lines(p)?.into_iter();
    let mut passports = vec![];
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
            .map(|p| (p[0].to_string(), p[1].to_string()))
            .collect::<HashMap<_, _>>();
        let passport = Passport::new(map);
        passports.push(passport);
    }
    Ok(passports)
}

#[derive(Debug, Clone)]
struct Passport {
    values: HashMap<String, String>,
}

impl Passport {
    fn new(values: HashMap<String, String>) -> Self {
        Self { values }
    }
}
