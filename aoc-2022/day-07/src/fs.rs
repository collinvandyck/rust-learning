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
    fn from<S: Into<String>>(s: S) -> Self {
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
    fn cd<S: Into<String>>(&mut self, s: S) {
        let s: String = s.into();
        if s.starts_with('/') {
            *self = Path::from(s)
        } else {
            self.0.push(s.into())
        }
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
}
