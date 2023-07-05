#![warn(clippy::all, clippy::pedantic)]

use home::home_dir;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::{cmp, env, fs};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HError {
    #[error("{0}")]
    IOError(#[from] io::Error),

    #[error("No home dir")]
    NoHomeDir,
}

const DEFAULT_TOPK: usize = 20;

fn main() -> Result<(), HError> {
    let Args { topk } = Args::parse();
    let dir = home_dir().ok_or(HError::NoHomeDir)?;
    let path = PathBuf::new().join(&dir).join(".zsh_history");
    let file = fs::File::open(path)?;
    let read = BufReader::new(file);
    let mut acc = Acc::new(topk);
    read.lines().flatten().for_each(|line| acc.parse(&line));
    println!("{acc}");
    Ok(())
}

/// arguments to the program
struct Args {
    topk: usize,
}

impl Args {
    fn parse() -> Args {
        let args = env::args().take(2).collect::<Vec<String>>();
        let topk = args.get(1).map_or(DEFAULT_TOPK, |arg| {
            if arg == "-h" || arg == "--help" {
                eprintln!("Usage: {} [topk]", args.get(0).unwrap());
                std::process::exit(0);
            }
            match arg.parse::<usize>() {
                Ok(n) => n,
                Err(e) => {
                    eprintln!(r#"Failed to parse topk for "{arg}": {e}"#);
                    std::process::exit(1);
                }
            }
        });
        Args { topk }
    }
}

/// Acc accumulates the results of parsing the history file
/// and summarizes them when printed
struct Acc {
    topk: usize,                // how many entries to display
    cmds: HashMap<String, u32>, // lookup for command counts
}

impl Acc {
    fn new(topk: usize) -> Self {
        Self {
            topk,
            cmds: HashMap::default(),
        }
    }
    fn parse(&mut self, line: &str) {
        lazy_static! {
            // example line:
            // : 1688435851:0;cmd arg1 arg2
            static ref LINE_RE: Regex = Regex::new(r"^(: \d+:\d;)? *(.*)$").unwrap();
        }
        if let Some(captures) = LINE_RE.captures(line) {
            if let Some(found) = captures.get(2) {
                found.as_str().split(' ').take(1).for_each(|cmd| {
                    *self.cmds.entry(cmd.to_string()).or_insert(0) += 1;
                });
            }
        }
    }
    fn summarize(&self) -> String {
        // convert the lookup to a vector and sort it
        let mut res: Vec<(&String, &u32)> = self.cmds.iter().collect();
        res.sort_by(|x, y| y.1.cmp(x.1));
        // take the topk
        res = res.into_iter().take(self.topk).collect::<Vec<_>>();
        // figure out padding for the topk results to make printing nice
        let max_len = res.iter().fold(0, |mx, s| cmp::max(mx, s.0.len()));
        // map the tuples into formatted strings and join them
        res.iter()
            .map(|(string, count)| {
                let padding = " ".repeat(max_len - string.len());
                format!("{string}{padding} : {count}")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let summary = self.summarize();
        write!(f, "{summary}")
    }
}
