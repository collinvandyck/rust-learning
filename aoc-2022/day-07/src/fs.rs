#![allow(dead_code)]

#[derive(Debug)]
pub struct FS {
    pwd: Path,
    root: FSObject,
}

impl FS {
    pub fn new() -> Self {
        Self {
            pwd: Path::from("/"),
            root: FSObject::new(),
        }
    }
    pub fn cd(&mut self, p: &str) {
        self.pwd.cd(p)
    }
}

#[test]
fn test_fs() {
    let mut fs = FS::new();
    println!("doing cd");
    fs.cd("/bar/baz");
    println!("done doing cd");
    dbg!(fs);
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

#[derive(Debug, PartialEq, Eq)]
pub struct Path(Vec<String>);

impl Path {
    pub fn from<S: Into<String>>(s: S) -> Self {
        let mut res = Path(vec![]);
        s.into()
            .trim_start_matches("/")
            .split('/')
            .map(str::to_string)
            .filter(|s| !s.is_empty())
            .for_each(|p| res.cd(p));
        res
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
    assert_eq!(Path::from("/bar"), Path(vec!["bar".to_string()]));
    assert_eq!(
        Path::from("bar/baz"),
        Path(vec!["bar".to_string(), "baz".to_string()])
    );

    let mut p = Path::from("/");
    p.cd("foo");
    assert_eq!(p, Path(vec!["foo".to_string()]));
    p.cd("bar");
    assert_eq!(p, Path(vec!["foo".to_string(), "bar".to_string()]));
    p.cd("..");
    assert_eq!(p, Path(vec!["foo".to_string()]));
}
