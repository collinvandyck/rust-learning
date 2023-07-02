#![allow(dead_code)]

use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{self, Write},
};

fn main() {
    let mut buf: Vec<u8> = vec![];

    // this is a reference to a trait type, which is a trait object.
    // it must be a pointer because the size of whatever the Write
    // implementation cannot be known usually at compile time.
    let writer: &mut dyn Write = &mut buf;

    writer.write(b"123").expect("write failure");
    println!("{}", buf.len());

    say_hello(&mut buf).expect("failed to write to buf");
    println!("{}", buf.len());

    let mut s = String::from_utf8(buf.clone()).expect("convert to utf8");
    s = s.trim().to_string();
    println!("{}", s);

    top_ten(&buf);
}

fn say_hello<W: Write>(out: &mut W) -> io::Result<()> {
    out.write_all(b"Hello, world!\n")?;
    out.flush()
}

fn top_ten<T: Debug + Hash + Eq>(vals: &Vec<T>) {
    let mut ht = HashMap::new();
    for ele in vals.iter() {
        let e = ht.entry(ele).or_insert(0);
        *e += 1;
    }
    dbg!(ht);
}

trait Vegetable {
    fn wilt(&self);
}

#[derive(Debug)]
enum SaladComponent {
    Lettuce,
    Tomato,
}

impl Vegetable for SaladComponent {
    fn wilt(&self) {
        println!("{:?} wilting!", self)
    }
}

struct Salad {
    veggies: Vec<Box<dyn Vegetable>>,
}

impl Salad {
    fn print(&self) {
        for v in &self.veggies {
            v.wilt();
        }
    }
}

#[test]
fn salad_test() {
    let f: Box<dyn Vegetable> = Box::new(SaladComponent::Lettuce);
    f.wilt();
}

fn print_veg(v: impl Vegetable) {
    v.wilt();
}
