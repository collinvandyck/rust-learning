fn main() {
    let mut tree = Tree::Empty;
    tree.add(5);
    dbg!(tree);
}

#[derive(Debug)]
enum Tree<T> {
    Empty,
    Value(Box<Node<T>>),
}

impl<T: Ord> Tree<T> {
    fn add(&mut self, val: T) {
        match self {
            Tree::Empty => {
                let node = Node {
                    item: val,
                    left: Tree::Empty,
                    right: Tree::Empty,
                };
                *self = Tree::Value(Box::new(node));
            }
            Tree::Value(node) => {}
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    item: T,
    left: Tree<T>,
    right: Tree<T>,
}
