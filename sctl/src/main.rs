use std::{
    error::Error,
    fmt::Debug,
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
        println!("Line: {record:?}");
        tree.add(record);
    }

    Ok(())
}

#[derive(Debug)]
struct Record {
    name: String,
    val: String,
}

impl FromStr for Record {
    type Err = SysctlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(2, ": ").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(SysctlError::ParseRecord(format!(
                "expected two parts from '{}' but got {}",
                s,
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

impl Tree {
    fn new() -> Self {
        Self::Leaf
    }
    fn add(&mut self, record: Record) {
        match self {
            Self::Leaf => {}
            Self::Node(node) => {}
        }
    }
}

struct Node {
    value: Record,
    children: Vec<Tree>,
}
