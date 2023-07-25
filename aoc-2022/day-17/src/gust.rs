#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gust {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Gusts(Vec<Gust>);

impl From<Vec<Gust>> for Gusts {
    fn from(value: Vec<Gust>) -> Self {
        Self(value)
    }
}

pub struct GustIter {
    gusts: Gusts,
    idx: usize,
}

impl Iterator for GustIter {
    type Item = Gust;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.gusts.0.get(self.idx).unwrap();
        self.idx = (self.idx + 1) % self.gusts.0.len();
        Some(*res)
    }
}

impl IntoIterator for Gusts {
    type Item = Gust;
    type IntoIter = GustIter;
    fn into_iter(self) -> Self::IntoIter {
        GustIter {
            gusts: self,
            idx: 0,
        }
    }
}

impl From<char> for Gust {
    fn from(value: char) -> Self {
        match value {
            '<' => Gust::Left,
            '>' => Gust::Right,
            _ => panic!("Invalid char: {value}"),
        }
    }
}

#[test]
fn test_gusts_iter() {
    let gusts: Gusts = "<>".chars().map(Gust::from).collect::<Vec<_>>().into();
    let mut iter = gusts.into_iter();
    assert_eq!(iter.next(), Some(Gust::Left));
    assert_eq!(iter.next(), Some(Gust::Right));
    assert_eq!(iter.next(), Some(Gust::Left));
    assert_eq!(iter.next(), Some(Gust::Right));
    assert_eq!(iter.next(), Some(Gust::Left));
    assert_eq!(iter.next(), Some(Gust::Right));
    assert_eq!(iter.next(), Some(Gust::Left));
    assert_eq!(iter.next(), Some(Gust::Right));
}
