#![allow(unused)]

trait Foo {
    fn foo(&self) -> String;
}

#[derive(Debug, foo_derive::Foo)]
struct Person {
    #[inst]
    name: &'static str,
}

impl Default for Person {
    fn default() -> Self {
        Self { name: "Collin" }
    }
}

fn main() {
    Person::default().hi();
    println!("{}", Person::default().foo());
}
