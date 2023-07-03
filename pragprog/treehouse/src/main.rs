#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
fn main() {
    println!("name:");
    let name = what_name();
    println!("Hello {name:?}");
}

fn what_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("read name");
    name
}
