use std::borrow::BorrowMut;

pub enum TreeMap<K, V> {
    Empty,
    NonEmpty(TreeNode<K, V>),
}

impl<K, V> TreeMap<K, V>
where
    K: Ord,
    V: PartialEq,
{
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn size(&self) -> usize {
        match *self {
            TreeMap::Empty => 0,
            TreeMap::NonEmpty(_) => todo!(),
        }
    }
    pub fn insert(&mut self, k: K, v: V) {
        let node = TreeNode::new(k, v);
        match self {
            TreeMap::Empty => *self = TreeMap::NonEmpty(node),
            TreeMap::NonEmpty(n) => n.insert(node),
        }
    }
}

pub struct TreeNode<K, V> {
    key: K,
    val: V,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> TreeNode<K, V>
where
    K: Ord,
    V: PartialEq,
{
    fn new(key: K, val: V) -> Self {
        let left = None;
        let right = None;
        Self {
            key,
            val,
            left,
            right,
        }
    }
    fn insert(&mut self, node: TreeNode<K, V>) {
        if node.key == self.key {
            self.val = node.val;
            return;
        }
        let child = if node.key < self.key {
            &mut self.left
        } else {
            &mut self.right
        };
        if let Some(child) = child {
            child.insert(node);
        } else {
            *child = Some(Box::new(node));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let mut t = TreeMap::new();
        assert_eq!(t.size(), 0);
        t.insert("foo", 32);
        assert_eq!(t.size(), 1);
        t.insert("foo", 32);
        assert_eq!(t.size(), 1);
    }
}
