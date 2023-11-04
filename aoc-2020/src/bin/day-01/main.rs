use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = (
        run("src/bin/day-01/example.txt", 2)?,
        run("src/bin/day-01/input.txt", 2)?,
    );
    println!("p1: {p1:?}");
    let p2 = (
        run("src/bin/day-01/example.txt", 3)?,
        run("src/bin/day-01/input.txt", 3)?,
    );
    println!("p2: {p2:?}");
    Ok(())
}

fn run(path: impl AsRef<Path>, n: usize) -> Result<i64> {
    let nums = file_to_lines(path.as_ref())?
        .into_iter()
        .map(|s| s.parse::<i64>())
        .collect::<StdResult<Vec<_>, _>>()?;
    let combos = combinations(nums, n);
    let res: i64 = combos
        .into_iter()
        .find(|v| v.iter().sum::<i64>() == 2020)
        .map(|v| v.iter().product())
        .ok_or_else(|| anyhow!("could not find answer"))?;
    Ok(res)
}
