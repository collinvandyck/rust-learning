#![allow(unused)]

use macros::{make_answer, AnswerFn};

#[derive(AnswerFn)]
struct Foo;

fn main() {
    println!("{}", answer());
}
