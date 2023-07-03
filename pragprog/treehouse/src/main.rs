#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;
fn main() {
    let visitors = ["bert", "collin", "steve"];
    println!("name:");
    let name = what_name();
    let mut found = false;
    for n in &visitors {
        if n == &name {
            println!("Welcome {name}");
            found = true;
        }
    }
    if !found {
        println!("Not welcome {name:?}");
    }
}

fn what_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("read name");
    name.trim().to_lowercase()
}
