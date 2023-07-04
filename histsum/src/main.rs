use regex::Regex;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HError {
    #[error("No home dir")]
    NoHomeDir,

    #[error("{0}")]
    IOError(#[from] io::Error),

    #[error("Could not parse: {0}")]
    ParseError(String),
}

impl HError {
    fn parse_err(err: &str) -> Result<(), HError> {
        Err(HError::ParseError(err.to_string()))
    }
}

// example line:
// : 1688435851:0;time ./target/release/histsum

fn main() -> Result<(), HError> {
    let dir = match std::env::home_dir() {
        Some(dir) => dir,
        _ => return Err(HError::NoHomeDir),
    };
    let path = Path::new(&dir).join(".zsh_history");
    let path = path.to_str().expect("path");
    let hist = fs::OpenOptions::new().read(true).open(path)?;
    let hist = BufReader::new(hist);
    let mut acc = Acc::new();
    hist.lines().for_each(|line| {
        if let Ok(line) = line {
            if line.len() == 0 {
                return;
            }
            match acc.accept(&line) {
                Err(e) => {
                    println!("Failed at: {}: {}", &line, e);
                    std::process::exit(1);
                }
                _ => (),
            }
        }
    });
    println!("Results: {acc:#?}");
    Ok(())
}

#[derive(Debug)]
struct Acc {
    re: Regex,
}

impl Acc {
    fn new() -> Self {
        let re = Regex::new("").unwrap();
        Self { re }
    }
    fn accept(&mut self, line: &String) -> Result<(), HError> {
        let line: Vec<char> = line.chars().collect();
        if line[0] != ':' {
            return HError::parse_err("expected :");
        }
        Ok(())
    }
}
