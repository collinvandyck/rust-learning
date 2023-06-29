fn main() {
    println!("Hello, world!");
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x: i32, y: i32) -> bool;
}

#[allow(dead_code)]
struct Broom {}

#[allow(unused)]
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {}
    fn hit_test(&self, x: i32, y: i32) -> bool {
        false
    }
}

#[allow(dead_code)]
struct Canvas {}
