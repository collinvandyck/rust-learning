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
    let mut visitor_list = vec![
        Visitor::new("bert", "welp hi there"),
        Visitor::new("collin", "Welcome"),
        Visitor::new("steve", "huh?"),
    ];
    loop {
        println!("name:");
        let name = what_name();
        let known = visitor_list.iter().find(|v| v.name == name);
        if let Some(visitor) = known {
            visitor.greet();
        } else {
            if name.is_empty() {
                break;
            }
            println!("Added {name} to visitor list");
            visitor_list.push(Visitor::new(&name, "New Friend"));
        }
    }
    println!("Final list of visitors: {visitor_list:?}");
}

fn what_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("read name");
    name.trim().to_lowercase()
}
