use std::ops::{Index, IndexMut};

pub enum Tree<K, V> {
    Dummy(Option<V>),
    Empty,
    NonEmpty(Node<K, V>),
}

impl<K, V> Tree<K, V>
where
    K: Ord,
    V: PartialEq,
{
    pub fn new() -> Self {
        Self::Dummy(None)
    }
    pub fn size(&self) -> usize {
        match *self {
            Tree::Dummy(_) => 0,
            Tree::Empty => 0,
            Tree::NonEmpty(_) => 1,
        }
    }
}

impl<K, V> Index<K> for Tree<K, V> {
    type Output = Option<V>;
    fn index(&self, index: K) -> &Option<V> {
        if let Tree::Dummy(o) = self {
            return o;
        }
        &None
    }
}

pub struct Node<K, V> {
    key: K,
    val: V,
    children: Vec<Node<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: Ord,
    V: PartialEq,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let t: Tree<&'static str, i32> = Tree::new();
        assert_eq!(t.size(), 0);
        assert_eq!(t["foo"], None);
    }
}
