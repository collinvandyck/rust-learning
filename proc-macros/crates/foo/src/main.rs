#![allow(unused)]

use macros::MyProc;

#[derive(MyProc)]
struct Foo {
    #[my_proc]
    name: String,
}

impl Foo {}

fn main() {}
