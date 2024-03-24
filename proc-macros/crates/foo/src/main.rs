#![allow(unused)]

use macros::MyProc;

#[derive(MyProc)]
struct Foo {
    #[inst]
    name: &'static str,
}

#[derive(MyProc)]
struct Bar {
    #[inst]
    name: &'static str,
}

fn main() {
    Foo { name: "Collin" }.hi();
    Bar { name: "Collin" }.hi();
}
