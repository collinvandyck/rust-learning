#![allow(dead_code, unused)]

use std::fmt::Display;

use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree {
    Root,
    Child(Vec<Node>),
}

impl Tree {
    fn insert_node(&mut self, mut insert: Node) {
        debug!(?insert, "Insert");
        match self {
            Tree::Root => *self = Tree::Child(vec![insert]),
            Tree::Child(ref mut nodes) => {
                for node in &mut *nodes {
                    let prefix = node.common_prefix(&insert);
                    match (prefix.len(), node.segments.len(), insert.segments.len()) {
                        (0, _, _) => { /*no prefix*/ }
                        (pl, nl, il) if pl < nl && pl < il => {
                            // prefix is a subset of both. make a new node and reparent them.
                            debug!("Reparenting both nodes");
                            let mut new_node = node.clone();
                            new_node.strip_prefix(&prefix);
                            insert.strip_prefix(&prefix);
                            let mut node_replace = Node::new(prefix.join("."), "");
                            node_replace.tree.insert_node(new_node);
                            node_replace.tree.insert_node(insert);
                            *node = node_replace;
                            return;
                        }
                        (pl, nl, il) => {
                            if pl < nl {
                                debug!("Prefix {prefix:?} is a subset of the node {node}");
                                // we should replace this node with insert.
                                // insert should have a child that is node
                                let mut new_node = node.clone();
                                new_node.strip_prefix(&prefix);
                                insert.tree.insert_node(new_node);
                                *node = insert;
                                return;
                            } else {
                                debug!("Prefix {prefix:?} is a subset of the insert {insert}");
                                // strip off the prefix of the insert, and then insert it into the
                                // node
                                insert.strip_prefix(&prefix);
                                node.tree.insert_node(insert);
                                return;
                            }
                        }
                    }
                }
                nodes.push(insert);
                nodes.sort_by_key(|node| node.segments.join("."));
            }
        }
    }
    pub fn insert(&mut self, key: impl AsRef<str>, val: impl AsRef<str>) {
        let key = key.as_ref().to_string();
        let val = val.as_ref().to_string();
        let insert = Node::new(key, val);
        self.insert_node(insert);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    segments: Vec<String>,
    value: String,
    tree: Tree,
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
            tree: Tree::Root,
        }
    }
    fn new_tree(key: impl AsRef<str>, value: impl AsRef<str>, tree: Tree) -> Self {
        let mut node = Self::new(key, value);
        node.tree = tree;
        node
    }
    fn common_prefix(&self, other: &Node) -> Vec<String> {
        self.segments
            .iter()
            .zip(other.segments.iter())
            .take_while(|(a, b)| a == b)
            .map(|(a, b)| a.to_string())
            .collect()
    }
    fn strip_prefix(&mut self, prefix: &[String]) {
        self.segments = self
            .segments
            .iter()
            .skip(prefix.len())
            .map(ToString::to_string)
            .collect();
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
        tree.insert("a.b.d", "baz");
        assert_eq!(
            tree,
            Tree::Child(vec![
                Node::new_tree(
                    "a.b",
                    "",
                    Tree::Child(vec![
                        //
                        Node::new("c", "foo"),
                        Node::new("d", "baz"),
                    ],)
                ),
                Node::new("c.d", "bar"),
            ]),
            "{:?}",
            dbg!(&tree),
        );
        tree.insert("c", "qux");
        assert_eq!(
            tree,
            Tree::Child(vec![
                Node::new_tree(
                    "a.b",
                    "",
                    Tree::Child(vec![
                        //
                        Node::new("c", "foo"),
                        Node::new("d", "baz"),
                    ],)
                ),
                Node::new_tree(
                    "c",
                    "qux",
                    Tree::Child(vec![
                        //
                        Node::new("d", "bar"),
                    ],)
                ),
            ]),
            "{:?}",
            dbg!(&tree),
        );
        tree.insert("a.b.c.d.e", "42");
        assert_eq!(
            tree,
            Tree::Child(vec![
                Node::new_tree(
                    "a.b",
                    "",
                    Tree::Child(vec![
                        //
                        Node::new_tree(
                            "c",
                            "foo",
                            Tree::Child(vec![
                                //
                                Node::new("d.e", "42"),
                            ],)
                        ),
                        Node::new("d", "baz"),
                    ],)
                ),
                Node::new_tree(
                    "c",
                    "qux",
                    Tree::Child(vec![
                        //
                        Node::new("d", "bar"),
                    ],)
                ),
            ]),
            "{:?}",
            dbg!(&tree),
        );
    }
}
