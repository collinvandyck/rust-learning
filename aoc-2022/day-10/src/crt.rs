pub struct Crt {
    rows: usize,
    cols: usize,
}

impl Crt {
    pub fn new() -> Self {
        Self { rows: 6, cols: 40 }
    }
    pub fn draw(&mut self, idx: usize, val: i64) {
        let col = idx % self.cols;
        if idx > 40 {
            return;
        }
        println!("idx: {idx} val: {val} col: {col}");
        if idx % self.cols == 0 {
            println!();
        }
    }
}
