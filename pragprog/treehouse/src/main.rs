#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    let visitor_list = vec![
        Visitor::new("bert", "welp hi there"),
        Visitor::new("collin", "Welcome"),
        Visitor::new("steve", "huh?"),
    ];
    loop {
        println!("name:");
        let name = what_name();
        let known = visitor_list.iter().find(|v| v.name == name);
        match known {
            Some(visitor) => visitor.greet(),
            _ => println!("Not welcome {name}"),
        }
    }
}

fn what_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("read name");
    name.trim().to_lowercase()
}
