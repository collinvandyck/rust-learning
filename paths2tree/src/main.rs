use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs::{self, DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

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
    render_samples()?;
    render_pwd(Path::new("."))
}

fn render_pwd(dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut tree = Tree::new();
    walk_dir(dir, &mut tree)?;
    println!("\nPWD:\n");
    tree.print(0, false);
    Ok(())
}

fn walk_dir(dir: &Path, tree: &mut Tree<String>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, tree)?;
        } else {
            let path = path.to_str().unwrap();
            let path = path.to_string();
            let lines: Vec<String> = path.split('/').map(|f| f.to_string()).collect();
            tree.add(lines);
        }
    }
    Ok(())
}

fn render_samples() -> Result<(), Box<dyn Error>> {
    let file_names = ["simple.txt", "input.txt"];
    for file_name in file_names {
        render(file_name)?;
    }
    Ok(())
}

fn render(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let file = BufReader::new(file);
    let mut tree = Tree::new();
    for line in file.lines() {
        let line = line?;
        let lines: Vec<String> = line.split('/').map(|f| f.to_string()).collect();
        tree.add(lines);
    }
    println!("\n{file_name}:\n");
    tree.print(0, false);
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
    fn print(&self, indent: usize, mut root_last: bool) {
        self.children.iter().enumerate().for_each(|(idx, tree)| {
            let is_last = idx == self.children.len() - 1;
            if indent == 0 {
                if is_last {
                    print!("└─")
                } else {
                    print!("├─");
                }
            } else {
                // we are in a subtree
                if root_last {
                    print!("   {}", "   ".repeat(indent - 1));
                } else {
                    print!("│  {}", "   ".repeat(indent - 1));
                }
                if is_last {
                    print!("└─")
                } else {
                    print!("├─");
                }
            }
            println!(" {}", tree.val.iter().next().unwrap());
            root_last = root_last || (is_last && indent == 0);
            tree.print(indent + 1, root_last);
        });
    }
}
