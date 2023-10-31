use anyhow::Result;
use std::{fs, result};

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
fn part_two() -> Result<()> {
    println!("Part two");
    let nums: Vec<i64> = fs::read_to_string("example.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            for z in &nums[2..] {
                if *x + *y + *z == 2020 {
                    println!("{}", *x * *y * *z);
                }
            }
        }
    }
    let nums: Vec<i64> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            for z in &nums[2..] {
                if *x + *y + *z == 2020 {
                    println!("{}", *x * *y * *z);
                }
            }
        }
    }
    Ok(())
}
fn part_one() -> Result<()> {
    println!("Part one");
    let nums: Vec<i64> = fs::read_to_string("example.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            if *x + *y == 2020 {
                println!("{}", *x * *y);
            }
        }
    }
    let nums: Vec<i64> = fs::read_to_string("input.txt")?
        .lines()
        .map(|l| l.parse::<i64>())
        .collect::<result::Result<Vec<_>, _>>()?;
    for x in &nums {
        for y in &nums[1..] {
            if *x + *y == 2020 {
                println!("{}", *x * *y);
            }
        }
    }
    Ok(())
}
