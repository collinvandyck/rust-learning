use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    let output = Command::new("sysctl").arg("-a").output()?;
    if !output.status.success() {
        bail!("sysctl failed");
    }
    let reader = BufReader::new(output.stdout.as_slice());
    let lines = reader.lines().collect::<std::result::Result<Vec<_>, _>>()?;
    let mut tree = prefix_tree::Tree::Root;
    for line in lines.into_iter().take(5) {
        println!("{line}");
        let parts = line.splitn(2, ": ").collect::<Vec<_>>();
        let key = parts[0];
        let val = parts[1];
        tree.insert(key, val);
    }
    dbg!(tree);
    Ok(())
}
