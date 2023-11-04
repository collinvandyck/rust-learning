use aoc_2020::prelude::*;

fn main() -> Result<()> {
    let p1 = (num_valid_pws("src/bin/day-01/example.txt")?,);
    println!("p1={p1:?}");
    Ok(())
}

fn num_valid_pws(p: impl AsRef<Path>) -> Result<usize> {
    todo!()
}
