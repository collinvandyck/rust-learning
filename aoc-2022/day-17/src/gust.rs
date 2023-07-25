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

impl From<char> for Gust {
    fn from(value: char) -> Self {
        match value {
            '<' => Gust::Left,
            '>' => Gust::Right,
            _ => panic!("Invalid char: {value}"),
        }
    }
}
