use std::rc::Rc;

fn main() {
    rc_panic_example();
}

fn rc_panic_example() {
    let mut r = Rc::new(String::new());
    let _r2 = r.clone();
    Rc::get_mut(&mut r).unwrap().push_str("foo");
    println!("{}", r);
}
