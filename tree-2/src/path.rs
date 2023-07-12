use std::str;

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
        match self.iter.next() {
            Some("") => Some("/"),
            Some(p) => Some(p),
            None => None,
        }
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
