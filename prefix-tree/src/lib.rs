#![allow(dead_code, unused)]

use std::fmt::Display;

use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    Root,
    Child(Vec<Node>),
}

impl Tree {
    pub fn insert(&mut self, key: impl AsRef<str>, val: impl AsRef<str>) {
        let key = key.as_ref().to_string();
        let val = val.as_ref().to_string();
        let to_insert = Node::new(key, val);
        debug!(?to_insert, "Inserting");
        match self {
            Tree::Root => {
                debug!("Replacing tree root");
                *self = Tree::Child(vec![to_insert]);
            }
            Tree::Child(ref mut nodes) => {
                // first, look for a prefix match.
                for node in &mut *nodes {
                    let prefix = node.common_prefix(&to_insert);
                    if !prefix.is_empty() {
                        debug!("Prefix of {node} and {to_insert}: {prefix:?}");
                    }
                }
                // if we got here, then there is no common prefix. just append it
                nodes.push(to_insert);
                nodes.sort_by_key(|node| node.segments.join("."));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    segments: Vec<String>,
    value: String,
    children: Vec<Tree>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segs = self.segments.join(".");
        write!(f, "{{{segs}: {}}}", self.value)
    }
}

impl Node {
    fn new(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        let segments = key.split(".").map(ToString::to_string).collect::<Vec<_>>();
        Self {
            segments,
            value,
            children: vec![],
        }
    }
    fn common_prefix(&self, other: &Node) -> Vec<String> {
        self.segments
            .iter()
            .zip(other.segments.iter())
            .take_while(|(a, b)| a == b)
            .map(|(a, b)| a.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_tree() {
        let mut tree = Tree::Root;
        tree.insert("a.b.c", "foo");
        assert_eq!(
            tree,
            Tree::Child(vec![
                //
                Node::new("a.b.c", "foo"),
            ])
        );
        tree.insert("c.d", "bar");
        assert_eq!(
            tree,
            Tree::Child(vec![
                //
                Node::new("a.b.c", "foo"),
                Node::new("c.d", "bar"),
            ])
        );
    }
}
