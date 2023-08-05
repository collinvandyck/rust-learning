use std::fmt::{Debug, Display};

pub enum TreeMap<K, V> {
    Empty,
    NonEmpty(TreeNode<K, V>),
}

impl<K, V> TreeMap<K, V>
where
    K: Ord + Debug,
    V: PartialEq + Debug,
{
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn size(&self) -> usize {
        self.iter().count()
    }
    pub fn insert(&mut self, k: K, v: V) {
        let node = TreeNode::new(k, v);
        match self {
            TreeMap::Empty => *self = TreeMap::NonEmpty(node),
            TreeMap::NonEmpty(n) => n.insert(node),
        }
    }
    pub fn iter<'a>(&'a self) -> TreeIter<'a, K, V> {
        let cur = match self {
            TreeMap::Empty => None,
            TreeMap::NonEmpty(node) => Some(node),
        };
        TreeIter::new(cur)
    }
}

impl<'a, K, V> IntoIterator for &'a TreeMap<K, V>
where
    K: Ord + Debug,
    V: PartialEq + Debug,
{
    type Item = &'a Entry<K, V>;
    type IntoIter = TreeIter<'a, K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry<K, V> {
    key: K,
    val: V,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, val: V) -> Self {
        Self { key, val }
    }
}

impl<K, V> Display for Entry<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.key, self.val)
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeNode<K, V> {
    entry: Entry<K, V>,
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
        let entry = Entry { key, val };
        Self { entry, left, right }
    }
    /// walk is not really needed, was just an intermediary state
    /// to make sure the tree looked right
    fn walk<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        fn walk_helper<K, V, F>(node: &TreeNode<K, V>, f: &mut F)
        where
            F: FnMut(&K, &V),
        {
            if let Some(node) = &node.left {
                walk_helper(node, f);
            }
            f(&node.entry.key, &node.entry.val);
            if let Some(node) = &node.right {
                walk_helper(node, f);
            }
        }
        walk_helper(self, &mut f);
    }
    fn insert(&mut self, node: TreeNode<K, V>) {
        if node.entry.key == self.entry.key {
            self.entry.val = node.entry.val;
            return;
        }
        let child = if node.entry.key < self.entry.key {
            &mut self.left
        } else {
            &mut self.right
        };
        match child {
            None => *child = Some(Box::new(node)),
            Some(ref mut child) => child.insert(node),
        }
    }
}

pub struct TreeIter<'a, K, V> {
    stack: Vec<&'a TreeNode<K, V>>,
}

impl<'a, K, V> TreeIter<'a, K, V> {
    fn new(cur: Option<&'a TreeNode<K, V>>) -> Self {
        let mut stack = vec![];
        if let Some(cur) = cur {
            stack.push(cur);
            let mut left = &cur.left;
            while let Some(ref node) = left {
                stack.push(node);
                left = &node.left;
            }
        }
        Self { stack }
    }
}

impl<'a, K, V> Iterator for TreeIter<'a, K, V>
where
    K: Debug,
{
    type Item = &'a Entry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let res = self.stack.pop().unwrap();
        if let Some(ref right) = res.right {
            self.stack.push(&right);
            let mut cur = right;
            while let Some(ref left) = cur.left {
                self.stack.push(&left);
                cur = left;
            }
        }
        Some(&res.entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_iter() {
        println!("here");
        let mut t = TreeMap::new();
        t.insert("foo", 32);
        t.insert("bar", 33);
        t.insert("zar", 34);
        t.insert("aar", 35);
        println!("here");
        let v = t.iter().count();
        assert_eq!(v, 4);
    }

    #[test]
    fn test_adds() {
        let mut t = TreeMap::new();
        assert_eq!(t.size(), 0);
        t.insert("foo", 32);
        t.insert("bar", 33);
        assert_eq!(t.size(), 2);
        t.insert("zar", 34);
        assert_eq!(t.size(), 3);
        t.insert("aar", 35);
        assert_eq!(t.size(), 4);
    }

    #[test]
    fn test_new_tree() {
        let mut t = TreeMap::new();

        println!("Here");
        assert_eq!(t.size(), 0);
        println!("Here2");
        t.insert("foo", 32);
        println!("Here3");
        assert_eq!(t.size(), 1);
        println!("Here4");
        t.insert("bar", 33);
        println!("Here5");
        assert_eq!(t.size(), 2);
        println!("Here6");
        t.insert("bar", 33);
        println!("Here7");
        assert_eq!(t.size(), 2);
    }

    #[test]
    fn test_tree_iter() {
        // empty
        let mut t: TreeMap<&'static str, i32> = TreeMap::new();
        let v = t.into_iter().collect::<Vec<_>>();
        assert!(v.is_empty());

        // has one entry
        t.insert("age", 48);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 48)]);

        // change the value of the entry
        t.insert("age", 49);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 49)]);

        // add a new entry with a greater key
        t.insert("age2", 50);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(v, vec![&Entry::new("age", 49), &Entry::new("age2", 50)]);

        // add a new entry with a lesser key
        t.insert("age3", 45);
        assert_eq!(t.size(), 3);
        let v = t.iter().collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                &Entry::new("age", 49),
                &Entry::new("age2", 50),
                &Entry::new("age3", 45),
            ]
        );
    }
}
