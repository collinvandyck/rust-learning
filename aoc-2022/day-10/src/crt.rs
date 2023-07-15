pub struct Crt {
    rows: usize,
    cols: usize,
}

impl Crt {
    pub fn new() -> Self {
        Self { rows: 6, cols: 40 }
    }
    pub fn draw(&mut self, idx: usize, val: i64) {
        print!("#");
        if idx % self.cols == 0 {
            println!();
        }
    }
}
