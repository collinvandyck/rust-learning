use anyhow::Result;

fn main() -> Result<()> {
    let lines = aoc_2020::file_to_lines("src/bin/day-01/example.txt")?;
    println!("Lines: {lines:?}");
    Ok(())
}
