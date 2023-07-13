use crate::prelude::*;

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
}

enum Node {
    Dir(String, Vec<Node>),
    File(String, u64),
}
