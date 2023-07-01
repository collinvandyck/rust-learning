#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
    let range = I32Range { start: 0, end: 10 };
    for x in range {
        println!("x: {}", x);
    }
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
