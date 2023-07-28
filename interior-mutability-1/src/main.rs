#![allow(unused_variables, dead_code)]
use std::rc::Rc;

fn main() {}

fn rc_panic_example() {
    let mut r = Rc::new(String::new());
    let _r2 = r.clone();
    // fails because we have another reference to the rc, so get_mut will return a None.
    Rc::get_mut(&mut r).unwrap().push_str("foo");
    println!("{}", r);
}
