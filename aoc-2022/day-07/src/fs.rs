use std::vec;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FS {
    pwd: Path,
    pub root: Node,
}

impl FS {
    pub fn new() -> Self {
        Self {
            root: Node::Dir("/".to_string(), vec![]),
            pwd: Path::new("/"),
        }
    }
    pub fn cd(&mut self, p: &str) {
        self.pwd.cd(p);
    }
    pub fn add(&mut self, node: Node) {
        let parts = self.pwd.parts();
        self.root.add(parts, node)
    }
}

impl<'a> IntoIterator for &'a FS {
    type Item = &'a Node;
    type IntoIter = FSIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        FSIter::new(self)
    }
}

pub struct FSIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'a> FSIter<'a> {
    fn new(fs: &'a FS) -> Self {
        let stack = vec![&fs.root];
        Self { stack }
    }
}

impl<'a> Iterator for FSIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        let node = self.stack.remove(0);
        if let Node::Dir(_, children) = node {
            for child in children {
                self.stack.insert(0, child);
            }
        }
        Some(node)
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Dir(String, Vec<Node>),
    File(String, u64),
}

impl Node {
    pub fn total_size(&self) -> u64 {
        match self {
            Node::Dir(_, children) => children.iter().map(|c| c.total_size()).sum(),
            Node::File(_, size) => *size,
        }
    }
    fn add(&mut self, mut path: Vec<String>, node: Node) {
        if let Some(part) = path.get(0).cloned() {
            path.remove(0);
            match self {
                Node::Dir(_, children) => {
                    let child = match children.iter_mut().find(|c| c.name() == &part) {
                        Some(child) => child,
                        None => {
                            children.push(Node::Dir(part.to_string(), vec![]));
                            children.last_mut().unwrap()
                        }
                    };
                    child.add(path, node);
                }
                _ => panic!("File found, expected dir"),
            }
        } else {
            // no more parts -- add the node
            if let Node::Dir(_, children) = self {
                children.push(node);
            }
        }
    }
    fn name(&self) -> &String {
        match self {
            Node::Dir(name, _) | Node::File(name, _) => name,
        }
    }
}
