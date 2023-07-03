#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome {} ({})", self.name, note);
                if self.age < 21 {
                    println!("Don't serve alcohol");
                }
            }
            VisitorAction::Probation => println!("{} is a probationary member", self.name),
            VisitorAction::Refuse => println!("{} is denied", self.name),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new(
            "collin",
            VisitorAction::AcceptWithNote {
                note: String::from("hi there"),
            },
            47,
        ),
        Visitor::new("steve", VisitorAction::Refuse, 38),
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
            visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
    }
    println!("Final list of visitors: {visitor_list:#?}");
}

fn what_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("read name");
    name.trim().to_lowercase()
}
