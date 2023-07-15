pub struct Crt {
    cols: usize,
    pos: i64,
}

impl Crt {
    pub fn new() -> Self {
        Self { cols: 40, pos: 0 }
    }
    pub fn draw(&mut self, _tick: usize, val: i64) {
        //println!("tick: {tick} val: {val} pos:{}", self.pos);

        // TODO: print the pixel or not
        if self.pos >= val - 1 && self.pos <= val + 1 {
            print!("{}", '#');
        } else {
            print!("{}", '.');
        }

        self.pos += 1;
        if self.pos == self.cols.try_into().unwrap() {
            self.pos = 0;
        }
        if self.pos == 0 {
            println!();
        }
    }
}
