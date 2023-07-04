use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;
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
    println!("Results: {acc}");
    Ok(())
}

#[derive(Debug)]
struct Acc {
    caps: HashMap<usize, u32>,
    cmds: HashMap<String, u32>,
}

// example line:
// : 1688435851:0;time ./target/release/histsum
impl Acc {
    fn new() -> Self {
        let caps = HashMap::new();
        let cmds = HashMap::new();
        Self { caps, cmds }
    }
    fn accept(&mut self, line: &String) -> Result<(), HError> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(: \d+:\d;)?(.*)$").unwrap();
            static ref CMD_RE: Regex = Regex::new(r"^(.*)\s*$").unwrap();
        }
        let res = LINE_RE.captures(line);
        let res = match res {
            Some(res) => res,
            None => return HError::parse_err("regex failed to match"),
        };
        *self.caps.entry(res.len()).or_insert(0) += 1;
        if res.get(1).is_none() {
            return Ok(());
        }
        match res.get(2) {
            Some(line) => {
                let line = line.as_str();
                line.split(' ').take(1).for_each(|cmd| {
                    *self.cmds.entry(cmd.to_string()).or_insert(0) += 1;
                })
            }
            None => {}
        }
        Ok(())
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v: Vec<(&String, &u32)> = self.cmds.iter().collect();
        v.sort_by(|x, y| y.1.cmp(x.1));
        v.iter().take(10).for_each(|(s, c)| {
            writeln!(f, "{}: {}", s, c).unwrap();
        });
        write!(f, "hi")
    }
}
