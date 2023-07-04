#![warn(clippy::all, clippy::pedantic)]

use home::home_dir;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::{cmp, env, fs};
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

const DEFAULT_TOPK: usize = 20;

fn main() -> Result<(), HError> {
    let Args { topk } = Args::parse();
    let hist = read_hist_file(".zsh_history")?;
    let mut acc = Acc::new(topk);
    hist.lines().flatten().for_each(|line| acc.parse(&line));
    println!("{acc}");
    Ok(())
}

fn read_hist_file(file_name: &str) -> Result<impl BufRead, HError> {
    let dir = home_dir().unwrap();
    let path = Path::new(&dir).join(file_name);
    let path = path.to_str().expect("path");
    let hist = fs::OpenOptions::new().read(true).open(path)?;
    let hist = BufReader::new(hist);
    Ok(hist)
}

/// arguments to the program
struct Args {
    topk: usize,
}

impl Args {
    fn parse() -> Args {
        let args = env::args().take(2).collect::<Vec<String>>();
        let topk = args
            .get(1)
            .map(|arg| {
                if arg == "-h" || arg == "--help" {
                    eprintln!("Usage: {} [topk]", args.get(0).unwrap());
                    std::process::exit(0);
                }
                match arg.parse::<usize>() {
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!(r#"Failed to parse topk for "{}": {}"#, arg, e);
                        std::process::exit(1);
                    }
                }
            })
            .unwrap_or(DEFAULT_TOPK);
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
            static ref LINE_RE: Regex = Regex::new(r"^(: \d+:\d;)?(.*)$").unwrap();
        }
        if let Some(captures) = LINE_RE.captures(line) {
            if let Some(found) = captures.get(2) {
                found.as_str().split(' ').take(1).for_each(|cmd| {
                    *self.cmds.entry(cmd.to_string()).or_insert(0) += 1;
                })
            }
        }
    }
    fn summarize(&self) -> String {
        let mut res = self
            .cmds
            .iter()
            .map(|(i, j)| (j, i)) // reverse the tuple
            .collect::<Vec<(&u32, &String)>>();
        res.sort_by(|x, y| y.0.cmp(x.0));
        // take the topk
        res = res.into_iter().take(self.topk).collect::<Vec<_>>();
        // figure out padding for the topk results
        let max_len = res.iter().fold(0, |mx, s| cmp::max(mx, s.1.len()));
        res.iter()
            .enumerate()
            .map(|(idx, (count, string))| {
                let padding = " ".repeat(max_len - string.len());
                let newline = if res.len() - idx == 1 { "" } else { "\n" };
                format!("{string}{padding} : {count}{newline}")
            })
            .collect()
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let summary = self.summarize();
        write!(f, "{summary}")
    }
}
