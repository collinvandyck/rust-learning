use std::str;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Path(pub Vec<String>);

impl Path {
    pub fn new(s: &str) -> Self {
        let mut v = vec![];
        PathIter::new(s).for_each(|p| {
            v.push(p.to_string());
        });
        Self(v)
    }
    pub fn parts(&self) -> Vec<String> {
        let mut res = self.0.clone();
        res.remove(0); // remove the leading "/" since we always start at "/"
        res
    }
    pub fn cd(&mut self, dir: &str) {
        let iter = PathIter::new(dir);
        for segment in iter {
            match segment {
                "/" => {
                    self.0 = vec!["/".to_string()];
                }
                ".." => {
                    if self.0.len() > 1 {
                        self.0.pop();
                    }
                }
                _ => self.0.push(segment.to_string()),
            };
        }
    }
}

#[test]
fn test_path() {
    let mut path = Path::new("/");
    assert_eq!(path, Path::new("/"));
    path.cd("foo/bar");
    assert_eq!(path, Path::new("/foo/bar"));
    path.cd("/baz/");
    assert_eq!(path, Path::new("/baz"));
    path.cd("../../../abc/def");
    assert_eq!(path, Path::new("/abc/def"));
}

pub struct PathIter<'a> {
    iter: str::Split<'a, char>,
    len: usize,
}

impl<'a> PathIter<'a> {
    pub fn new(s: &'a str) -> Self {
        PathIter {
            iter: s.trim_end_matches('/').split('/'),
            len: s.len(),
        }
    }
    #[cfg(test)]
    fn to_vec(self) -> Vec<&'a str> {
        self.collect::<Vec<&str>>()
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.iter.next().map(|s| if s == "" { "/" } else { s })
    }
}

#[test]
fn test_path_iter() {
    assert_eq!(PathIter::new("").to_vec(), vec![] as Vec<&str>,);
    assert_eq!(PathIter::new("/").to_vec(), vec!["/"],);
    assert_eq!(PathIter::new("/foo").to_vec(), vec!["/", "foo"],);
    assert_eq!(PathIter::new("/foo/bar").to_vec(), vec!["/", "foo", "bar"],);
    assert_eq!(PathIter::new("foo/bar").to_vec(), vec!["foo", "bar"],);
    assert_eq!(
        PathIter::new("foo/bar/baz/").to_vec(),
        vec!["foo", "bar", "baz"],
    );
}
