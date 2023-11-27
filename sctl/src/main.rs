#![allow(unused)]
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    io::{self, BufRead, BufReader},
    process::Command,
    str::FromStr,
};

#[derive(thiserror::Error, Debug)]
enum SysctlError {
    #[error("sysctl failed")]
    SysctlFailed { stdout: Vec<u8>, stderr: Vec<u8> },
    #[error("io error: {0}")]
    IO(io::Error),
    #[error("parse record: {0}")]
    ParseRecord(String),
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("sysctl").arg("-a").output()?;
    if !output.status.success() {
        return Err(SysctlError::SysctlFailed {
            stdout: output.stdout,
            stderr: output.stderr,
        }
        .into());
    }
    let stdout = BufReader::new(output.stdout.as_slice());
    let mut tree = Tree::new();
    for record in stdout
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| SysctlError::IO(err))?
        .into_iter()
        .map(|s| s.parse::<Record>())
        .collect::<Result<Vec<_>, _>>()?
    {
        tree.add(record);
    }

    println!("Tree:\n{tree}");
    Ok(())
}

#[derive(Debug)]
struct Record {
    name: String,
    val: String,
}

impl Record {
    fn merge(other: &Record) {}
}

impl FromStr for Record {
    type Err = SysctlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(2, ": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(SysctlError::ParseRecord(format!(
                "expected two parts from '{s}' but got {}",
                parts.len()
            )));
        }
        Ok(Self {
            name: parts[0].to_string(),
            val: parts[1].to_string(),
        })
    }
}

enum Tree {
    Leaf,
    Node(Node),
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0);
        write!(f, "{buf}")
    }
}

impl Tree {
    fn new() -> Self {
        Self::Leaf
    }
    fn print(&self, buf: &mut String, depth: usize) {
        match self {
            Tree::Leaf => {}
            Tree::Node(Node { record, children }) => {
                let indent = "  ".repeat(depth);
                buf.push_str(&format!("{indent}{}: {}\n", record.name, record.val));
                for child in children {
                    child.print(buf, depth + 1);
                }
            }
        }
    }
    fn add(&mut self, record: Record) {
        match self {
            Self::Leaf => *self = Self::Node(Node::new(record)),
            Self::Node(ref mut node) => {
                let crec = &mut node.record;
            }
        }
    }
}

struct Node {
    record: Record,
    children: Vec<Tree>,
}

impl Node {
    fn new(record: Record) -> Self {
        Self {
            record,
            children: vec![],
        }
    }
}
