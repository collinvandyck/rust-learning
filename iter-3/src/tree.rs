pub enum Tree<K, V> {
    Empty,
    NonEmpty(Node<K, V>),
}

impl<K, V> Tree<K, V>
where
    K: Ord,
    V: PartialEq,
{
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn size(&self) -> usize {
        match *self {
            Tree::Empty => 0,
            Tree::NonEmpty(_) => 1,
        }
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
    }
}
