use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
///    └─ six
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);
    let mut tree = Tree::new();
    for line in file.lines() {
        let line = line?;
        let lines: Vec<String> = line.split('/').map(|f| f.to_string()).collect();
        tree.add(lines);
    }
    tree = dbg!(tree);
    tree.print(0);
    Ok(())
}

#[derive(Debug)]
struct Tree<T> {
    val: Option<T>,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq<T> + Debug + Display,
{
    fn new() -> Self {
        Self {
            val: None,
            children: vec![],
        }
    }
    fn add(&mut self, parts: Vec<T>) {
        let mut tree = self;
        for part in parts {
            let pos = tree.children.iter().position(|x| match &x.val {
                Some(val) => *val == part,
                None => false,
            });
            match pos {
                None => {
                    // not found. append to the children
                    let mut sub_tree = Tree::new();
                    sub_tree.val = Some(part);
                    tree.children.push(sub_tree);
                    tree = tree.children.last_mut().unwrap();
                }
                Some(pos) => {
                    // found it
                    let sub_tree = tree.children.get_mut(pos).unwrap();
                    tree = sub_tree;
                }
            }
        }
    }
    /// ├─ one
    /// │  ├─ two
    /// │  └─ three
    /// │     └─ four
    /// └─ five
    ///    └─ six
    fn print(&self, indent: usize) {
        self.children.iter().enumerate().for_each(|(idx, tree)| {
            let last = idx == self.children.len();
            let val = tree.val.iter().next();
            // first print out the leading tree part.
            if indent == 0 || idx == 0 {}
            print!("├─ {}", val.unwrap());
            println!();
            tree.print(indent + 1);
        });
    }
}
