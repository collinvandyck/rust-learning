mod path;

mod prelude {
    pub use crate::path::*;
}

use prelude::*;

fn main() {
    let mut t = Tree::new();
    t.root.mkdir_p("/foo/bar/baz");
    t.cd("collin");
    dbg!(t);
}

#[derive(Debug)]
struct Tree {
    pwd: Path,
    root: Node,
}

impl Tree {
    fn new() -> Self {
        Self {
            pwd: Path::new("/"),
            root: Node::Dir("/".to_string(), vec![]),
        }
    }
    fn cd(&mut self, dir: &str) {
        self.pwd.cd(dir);
    }
}

#[derive(Debug)]
enum Node {
    File(String),
    Dir(String, Vec<Node>),
}

impl Node {
    fn mkdir_p(&mut self, mut dirs: &str) {
        if dirs.starts_with('/') {
            dirs = &dirs[1..]
        }
        if dirs.is_empty() {
            return;
        }
        let (dir, dirs) = match dirs.find('/') {
            Some(idx) => (&dirs[..idx], &dirs[idx..]),
            None => (dirs, ""),
        };
        if let Node::Dir(_, ref mut children) = self {
            for child in children.iter_mut() {
                if let Node::Dir(name, _) = child {
                    if name == &dir {
                        child.mkdir_p(dirs);
                        return;
                    }
                }
            }
            let new_node = Node::Dir(dir.to_string(), vec![]);
            children.push(new_node);
            let new_node = children.last_mut().unwrap();
            new_node.mkdir_p(dirs);
        }
    }
}
