pub struct Crt {
    cols: usize,
    pos: usize,
}

impl Crt {
    pub fn new() -> Self {
        Self { cols: 40, pos: 0 }
    }
    pub fn draw(&mut self, tick: usize, val: i64) {
        self.pos += 1;
        if self.pos > self.cols {
            self.pos = 1;
        }
        println!("tick: {tick} val: {val} pos:{}", self.pos);
        if self.pos % self.cols == 0 {
            println!();
        }
    }
}
