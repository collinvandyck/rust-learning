#![allow(unused)]

use macros::{as_is, my_proc, MyProc};

my_proc!();
#[derive(MyProc, Default, Debug)]
struct Foo {
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
    my_proc();
    says_hello();
}
