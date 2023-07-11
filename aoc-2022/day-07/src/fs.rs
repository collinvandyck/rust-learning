#![allow(dead_code)]

use crate::prelude::*;

#[derive(Debug)]
pub enum FS {
    File(String, u64),
    Dir(String, Vec<FS>),
}

impl FS {
    pub fn new() -> Self {
        FS::Dir("/".to_string(), vec![])
    }
    pub fn add_dir(&mut self, name: &str) {
        match self {
            FS::File(_, _) => panic!("file"),
            FS::Dir(_, children) => {
                let name = name.to_string();
                let exists = children.iter().any(|c| match c {
                    FS::Dir(n, _) if n == c.name() => true,
                    _ => false,
                });
                if !exists {
                    children.push(FS::Dir(name.to_string(), vec![]))
                }
            }
        }
    }
    fn name(&self) -> &String {
        match self {
            FS::File(name, _) => name,
            FS::Dir(name, _) => name,
        }
    }
}

#[test]
fn test_fs() {
    let mut fs = FS::new();
    fs.add_dir("foo");
    dbg!(fs);
}

#[derive(Debug, PartialEq, Eq)]
struct Path(Vec<String>);

impl Path {
    fn from(s: &str) -> Self {
        let mut parts = s
            .trim_start_matches("/")
            .split('/')
            .map(str::to_string)
            .filter(|s| s.is_empty())
            .collect::<Vec<_>>();
        if parts.len() == 1 && parts.get(0) == Some(&"".to_string()) {
            parts = vec![];
        }
        Self(parts)
    }
}

#[test]
fn test_path() {
    assert_eq!(Path::from(""), Path(vec![]));
    assert_eq!(Path::from("/"), Path(vec![]));
}
