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

impl HError {
    fn parse_err(err: &str) -> Result<(), HError> {
        Err(HError::ParseError(err.to_string()))
    }
}

const DEFAULT_TOPK: usize = 20;

struct Args {
    topk: usize,
}

fn main() -> Result<(), HError> {
    let Args { topk } = parse_args();
    let hist = read_hist_file(".zsh_history")?;
    let mut acc = Acc::new(topk);
    hist.lines().for_each(|line| {
        if let Ok(line) = line {
            match acc.accept(&line) {
                Err(e) => {
                    println!("Failed at: {}: {}", &line, e);
                    std::process::exit(1);
                }
                _ => (),
            }
        }
    });
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

fn parse_args() -> Args {
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

/// Acc accumulates the results of parsing the history file
/// and summarizes them when printed
struct Acc {
    topk: usize,                // how many entries to display
    cmds: HashMap<String, u32>, // lookup for command counts
}

impl Acc {
    fn new(topk: usize) -> Self {
        let cmds = HashMap::default();
        Self { topk, cmds }
    }
    fn accept(&mut self, line: &String) -> Result<(), HError> {
        lazy_static! {
            // : 1688435851:0;cmd arg1 arg2
            static ref LINE_RE: Regex = Regex::new(r"^(: \d+:\d;)?(.*)$").unwrap();
        }
        let res = LINE_RE.captures(line);
        let res = match res {
            Some(res) => res,
            None => return HError::parse_err("regex failed to match"),
        };
        if let None = res.get(1) {
            return Ok(());
        }
        // get the command part
        if let Some(found) = res.get(2) {
            found.as_str().split(' ').take(1).for_each(|cmd| {
                *self.cmds.entry(cmd.to_string()).or_insert(0) += 1;
            })
        }
        Ok(())
    }
}

impl Display for Acc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v: Vec<(&String, &u32)> = self.cmds.iter().collect();
        v.sort_by(|x, y| y.1.cmp(x.1));
        v = v.into_iter().take(self.topk).collect();
        let max = v.iter().fold(0, |s, c| cmp::max(s, c.0.len()));
        let mut res = String::new();
        v.iter()
            .take(self.topk)
            .enumerate()
            .for_each(|(i, (s, c))| {
                let mut padding: Option<String> = None;
                let str_len = s.len();
                if max > str_len {
                    padding = Some(" ".repeat(max - str_len));
                }
                let padding: String = padding.unwrap_or("".to_string());
                let mut line = format!("{}{} : {}", s, padding, c);
                if i < v.len() - 1 {
                    line.push_str("\n");
                }
                res.push_str(&line);
            });
        write!(f, "{res}")
    }
}
