pub struct PathIter<'a> {
    path: &'a str,
}

impl<'a> PathIter<'a> {
    pub fn new(s: &'a str) -> Self {
        PathIter { path: s }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.path.starts_with('/') {
            self.path = &self.path[1..];
        }
        if self.path.is_empty() {
            return None;
        }
        let to = match self.path.find('/') {
            Some(idx) => idx,
            None => self.path.len(),
        };
        let res = &self.path[..to];
        self.path = &self.path[to..];
        Some(res)
    }
}

#[test]
fn test_path_iter() {
    assert_eq!(
        vec!["foo", "bar"],
        PathIter::new("/foo/bar").into_iter().collect::<Vec<&str>>()
    );
}
