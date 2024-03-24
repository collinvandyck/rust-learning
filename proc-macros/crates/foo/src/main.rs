#![allow(unused)]

#[derive(foo_derive::IsFoo)]
struct Foo {
    #[inst]
    name: &'static str,
}

fn main() {
    Foo { name: "Collin" }.hi();
}
