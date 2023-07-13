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
        self.mkdirs_p();
    }
    // ensures that the FS has the directories listed by pwd
    fn mkdirs_p(&mut self) {
        println!("mkdirs_p");
        let parts: Vec<&String> = self.pwd.0.iter().collect::<Vec<_>>();
        let mut parts = parts.iter();
        let mut cur = &mut self.root;
        loop {
            match parts.next() {
                Some(part) => {
                    if let Node::Dir(name, children) = cur {
                        if &name != part {
                            break;
                        }
                        println!("cur={cur:?}");
                    } else {
                        // no match
                        break;
                    }
                }
                None => break,
            }
        }
    }
}

#[derive(Debug)]
enum Node {
    Dir(String, Vec<Node>),
    File(String, u64),
}

impl Node {
    fn mkdirs_p(&mut self, mut parts: Vec<String>) {
        if parts.is_empty() {
            return;
        }
        let part = parts.remove(0);
        match self {
            Node::Dir(name, children) if name == &part => {
                if let Some(part) = parts.get(0) {
                    let found = children.iter_mut().find(|child| child.name() == part);
                    let child = match found {
                        Some(node) => node,
                        None => {
                            let new_child = Node::Dir(part.to_string(), vec![]);
                            children.push(new_child);
                            children.iter_mut().last().unwrap()
                        }
                    };
                    child.mkdirs_p(parts);
                }
            }
            Node::Dir(_, children) => {
                let new_child = Node::Dir(part.to_string(), vec![]);
                children.push(new_child);
                let child = children.iter_mut().last().unwrap();
                child.mkdirs_p(parts);
            }
            Node::File(_, _) => panic!("file encountered"),
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
