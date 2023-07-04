use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HError {
    #[error("No home dir")]
    NoHomeDir,

    #[error("{0}")]
    IOError(#[from] io::Error),
}

fn main() -> Result<(), HError> {
    let dir = match std::env::home_dir() {
        Some(dir) => dir,
        _ => return Err(HError::NoHomeDir),
    };
    let path = Path::new(&dir).join(".zsh_history");
    let path = path.to_str().expect("path");
    let hist = fs::OpenOptions::new().read(true).open(path)?;
    let hist = BufReader::new(hist);
    let mut max = 0;
    for (id, line) in hist.lines().enumerate() {
        max = id + 1;
        let line = line?;
        println!("{line}");
    }
    println!("Total: {max} entries.");
    Ok(())
}
