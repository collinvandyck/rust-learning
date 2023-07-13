use crate::prelude::*;

#[derive(Debug)]
pub struct FS {
    pwd: Path,
    root: Node,
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

#[derive(Debug)]
pub enum Node {
    Dir(String, Vec<Node>),
    File(String, u64),
}

impl Node {
    fn add(&mut self, mut parts: Vec<String>, node: Node) {
        if let Some(part) = parts.get(0).cloned() {
            parts.remove(0);
            match self {
                Node::Dir(_, children) => {
                    let child = match children.iter_mut().find(|c| c.name() == &part) {
                        Some(child) => child,
                        None => {
                            children.push(Node::Dir(part.to_string(), vec![]));
                            children.last_mut().unwrap()
                        }
                    };
                    child.add(parts, node);
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
    fn mkdirs_p(&mut self, mut parts: Vec<String>) {
        if let Some(part) = parts.get(0).cloned() {
            parts.remove(0);
            if let Node::Dir(name, children) = self {
                if name == &part {
                    if parts.is_empty() {
                        return;
                    }
                    let part = parts.remove(0);
                    let next = children.iter_mut().find(|c| c.name() == &part);
                    let next = match next {
                        Some(next) => next,
                        _ => {
                            children.push(Node::Dir(part.to_string(), vec![]));
                            children.last_mut().unwrap()
                        }
                    };
                    next.mkdirs_p(parts);
                } else {
                    // the name didn't match. add a child directory
                    children.push(Node::Dir(part.to_string(), vec![]));
                    let next = children.last_mut().unwrap();
                    next.mkdirs_p(parts);
                }
            }
        }
    }
    fn name(&self) -> &String {
        match self {
            Node::Dir(name, _) | Node::File(name, _) => name,
        }
    }
}

#[test]
fn test_node_mkdirs_p() {
    let mut node = Node::Dir("/".to_string(), vec![]);
    node.mkdirs_p(vec!["/".to_string(), "foo".to_string(), "bar".to_string()]);
    dbg!(node);
}
