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
            Tree::NonEmpty(_) => todo!(),
        }
    }
    pub fn insert(&mut self, k: K, v: V) {
        let node = Node::new(k, v);
        match self {
            Tree::Empty => *self = Tree::NonEmpty(node),
            Tree::NonEmpty(n) => n.insert(node),
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
    fn new(key: K, val: V) -> Self {
        let children = vec![];
        Self { key, val, children }
    }
    fn insert(&mut self, node: Node<K, V>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let mut t = Tree::new();
        assert_eq!(t.size(), 0);
        t.insert("foo", 32);
        assert_eq!(t.size(), 0);
    }
}
