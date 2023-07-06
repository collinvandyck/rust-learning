use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/// one/two
/// one/three/four
/// five/six
/// and have it render the tree visually:
///
/// ├─ one
/// │  ├─ two
/// │  └─ three
/// │     └─ four
/// └─ five
/// └─ six
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);
    let mut tree = Tree::new();
    for line in file.lines() {
        let line = line?;
        tree.add(&line);
    }
    Ok(())
}

struct Tree(Vec<Node>);

struct Node {
    val: String,
    tree: Tree,
}

impl Node {
    fn new(val: String) -> Self {
        Self {
            val,
            tree: Tree::new(),
        }
    }
}

impl Tree {
    fn new() -> Self {
        Self(vec![])
    }
    fn add(&mut self, p: &str) {
        let parts: Vec<&str> = p.split('/').into_iter().rev().collect();
        self.add_parts(parts);
    }

    fn add_parts(&mut self, mut parts: Vec<&str>) {
        let mut tree = self;
        for part in parts {
            let part = part.to_string();
            let children = &mut tree.0;
        }
        /*
        for part in parts {
            let part = part.to_string();
            let node = &mut tree.0.iter().find(|p| p.val == part);
            match node {
                None => {
                    let mut node = Node::new(part);
                    tree.0.push(Node::new(part));
                    tree = &mut node.tree;
                }
                Some(node) => {
                    parts.remove(0);
                    node.tree.add_parts(parts);
                }
            }
        }
        */
    }
}
