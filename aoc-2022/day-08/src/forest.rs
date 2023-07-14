use std::slice::SliceIndex;

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
    pub fn visible(&self) -> u32 {
        let mut count: u32 = 0;
        let rows = self.rows();
        let cols = self.cols();
        for row in 0..rows {
            for col in 0..cols {
                if self.tree_visible(row, col) {
                    count += 1;
                }
            }
        }
        count
    }
    fn tree_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 {
            return true;
        }
        if row == self.rows() - 1 {
            return true;
        }
        if col == 0 {
            return true;
        }
        if col == self.cols() - 1 {
            return true;
        }
        self.higher_than_neighbors(row, col)
    }
    fn higher_than_neighbors(&self, row: usize, col: usize) -> bool {
        let height = self.height_at(row, col);
        println!("Height at {row}x{col} is {height}");
        false
    }
    fn height_at(&self, row: usize, col: usize) -> usize {
        let r = self.trees.get(row).unwrap();
        r.get(col).unwrap().0
    }
    fn rows(&self) -> usize {
        self.trees.len()
    }
    fn cols(&self) -> usize {
        if self.rows() == 0 {
            return 0;
        }
        self.trees.get(0).unwrap().len()
    }
}

#[derive(Debug)]
pub struct Tree(pub usize);
