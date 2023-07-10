fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
struct Ship(Vec<Stack>);

struct Stack(Vec<Crate>);

struct Crate(char);
