fn main() {
    let range = I32Range { start: 0, end: 10 };
    for x in range {
        println!("x: {}", x);
    }

    let mut tree = BinaryTree::<i32>::Empty;
    tree.add(1);
    tree.add(3);
    tree.add(5);
    tree.add(4);
    tree.add(6);
    tree.add(2);

    for n in &tree {
        println!("{}", n);
    }
    for n in &tree {
        println!("{}", n);
    }
}

enum BinaryTree<T> {
    Empty,
    Tree(Box<TreeNode<T>>),
}

impl<T> BinaryTree<T>
where
    T: PartialOrd + PartialEq,
{
    fn add(&mut self, val: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Tree(Box::new(TreeNode {
                    val: val,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::Tree(node) => {
                if val < node.val {
                    node.left.add(val);
                } else {
                    node.right.add(val);
                }
            }
        }
    }

    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T>
where
    T: PartialOrd + PartialEq,
{
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

// generic params and lifetimes on the left. in this case, the lifetime of the
// iterator is the same as the lifetime of the parameter T: 'a
impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::Tree(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T: 'a> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // find the node this iteration must produce, or finish the iteration.
        let node = self.unvisited.pop()?;

        // after node, the next thing we must produce must be the leftmost
        // child in "node"'s right subtree, so we must push the path down from
        // here.
        self.push_left_edge(&node.right);

        // finally, produce a reference
        Some(&node.val)
    }
}

struct TreeNode<T> {
    val: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let result = Some(self.start);
            self.start += 1;
            result
        }
    }
}
