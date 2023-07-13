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
