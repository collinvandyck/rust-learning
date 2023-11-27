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
        .into_iter()
        .take(2)
    {
        tree.add(record);
    }

    println!("Tree:\n{tree}");
    Ok(())
}

enum BetterTree {
    Empty,
    Node(Vec<BetterRecord>),
}

struct BetterRecord {
    name: String,
    segments: Vec<String>,
    val: String,
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
                buf.push_str(&format!("{indent}{:?}: {}\n", record.segments, record.val));
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
                let node_rec: &mut Record = &mut node.record;
                let node_children: &mut Vec<Tree> = &mut node.children;

                // we are adding the record to the node of this tree.
                // if the record has a segment prefix of this node, we need to break this node up
                // so that it can accommodate the record as a child.

                match record.relationship(node_rec) {
                    Relationship::Child => todo!(),
                    Relationship::None => {
                        // node rec should live on the same level
                    }
                }
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

#[derive(Debug)]
struct Record {
    name: String,
    segments: Vec<String>,
    val: String,
}

enum Relationship {
    Child,
    None,
}

impl Record {
    fn relationship(&self, other: &Record) -> Relationship {
        Relationship::None
    }
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
            segments: parts[0]
                .trim()
                .split(".")
                .map(ToString::to_string)
                .collect(),
            name: parts[0].to_string(),
            val: parts[1].to_string(),
        })
    }
}
