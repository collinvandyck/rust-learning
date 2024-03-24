#![allow(unused)]

use macros::{as_is, MyProc};

#[derive(MyProc, Default, Debug)]
struct Foo {
    #[my_proc]
    name: String,
}

#[as_is(hi => "foo")]
fn says_hello() {
    println!("says_hello()");
}

impl Foo {
    fn new() -> Self {
        Self::default()
    }
}

fn main() {
    let f = Foo {
        name: String::from("foo"),
    };
    println!("mpd: {}", my_proc_derive());
    println!("foo: {f:#?}");
    says_hello();
}
