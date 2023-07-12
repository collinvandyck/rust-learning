#![allow(dead_code)]

use crate::prelude::*;

pub struct FS {
    pwd: Path,
    root: FSObject,
}

impl FS {
    fn new() -> Self {
        Self {
            pwd: Path::from("/"),
            root: FSObject::new(),
        }
    }
}

#[derive(Debug)]
pub enum FSObject {
    File(String, u64),
    Dir(String, Vec<FSObject>),
}

impl FSObject {
    pub fn new() -> Self {
        FSObject::Dir("/".to_string(), vec![])
    }
    fn name(&self) -> &String {
        match self {
            FSObject::File(name, _) => name,
            FSObject::Dir(name, _) => name,
        }
    }
}

#[test]
fn test_fs() {
    let mut fs = FSObject::new();
    dbg!(fs);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Path(Vec<String>);

impl Path {
    pub fn from<S: Into<String>>(s: S) -> Self {
        let s = s.into();
        let mut parts = s
            .trim_start_matches("/")
            .split('/')
            .filter(|s| s.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        if parts.len() == 1 && parts.get(0) == Some(&"".to_string()) {
            parts = vec![];
        }
        Self(parts)
    }
    pub fn cd<S: Into<String>>(&mut self, s: S) {
        let s: String = s.into();
        if s.starts_with('/') {
            *self = Path::from(s)
        } else if &s[..] == ".." {
            self.0.pop();
        } else {
            self.0.push(s.into());
        }
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn first(&self) -> &str {
        self.0.first().unwrap()
    }
}

#[test]
fn test_path() {
    assert_eq!(Path::from(""), Path(vec![]));
    assert_eq!(Path::from("/"), Path(vec![]));

    let mut p = Path::from("/");
    p.cd("foo");
    assert_eq!(p, Path(vec!["foo".to_string()]));
    p.cd("bar");
    assert_eq!(p, Path(vec!["foo".to_string(), "bar".to_string()]));
    p.cd("..");
    assert_eq!(p, Path(vec!["foo".to_string()]));
}
