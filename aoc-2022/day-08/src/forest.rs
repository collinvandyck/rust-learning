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
    pub fn scenic_score(&self) -> u32 {
        let mut score = 0;
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let height = self.height_at(row, col);
                let ranges = self.neighbor_ranges(row, col);
                ranges
                    .iter()
                    .map(|rg| {
                        let mut score: u32 = 0;
                        for h in rg.iter().map(|(r, c)| self.height_at(*r, *c)) {
                            score += 1;
                            if h >= height {
                                break;
                            }
                        }
                        score
                    })
                    .reduce(|acc, x| acc * x)
                    .into_iter()
                    .for_each(|s| score = u32::max(s, score));
            }
        }
        score
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
        let ranges = self.neighbor_ranges(row, col);
        ranges.iter().any(|rg| {
            rg.iter()
                .map(|(r, c)| self.height_at(*r, *c))
                .all(|h| h < height)
        })
    }
    fn neighbor_ranges(&self, row: usize, col: usize) -> Vec<Vec<(usize, usize)>> {
        let mut res = vec![];
        res.push((0..row).rev().map(|r| (r, col)).collect());
        res.push((row + 1..self.rows()).map(|r| (r, col)).collect());
        res.push((0..col).rev().map(|c| (row, c)).collect());
        res.push((col + 1..self.cols()).map(|c| (row, c)).collect());
        res
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
