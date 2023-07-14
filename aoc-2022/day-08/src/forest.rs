#[derive(Debug)]
pub struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn new() -> Self {
        Self { trees: vec![] }
    }
    pub fn add_line(&mut self, line: Vec<Tree>) {
        self.trees.push(line);
    }
}

#[derive(Debug)]
pub struct Tree(pub usize);
