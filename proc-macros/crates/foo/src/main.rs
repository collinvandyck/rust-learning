#![allow(unused)]
use macros::{as_is, AnswerFn, Barks};

struct Bar {
    name: String,
}

#[derive(AnswerFn, Barks, Default)]
struct Foo {
    name: String,
}

#[as_is(hi => "foo")]
fn says_hello() {
    println!("hello");
}

impl Foo {
    fn new() -> Self {
        Self::default()
    }
}

fn main() {
    println!("{}", answer());
    println!("{}", barks());
    says_hello();
}
