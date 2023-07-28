#![allow(unused_variables, dead_code, unused_imports)]
use std::rc::Rc;

fn main() {}

mod rc {
    use std::rc::Rc;
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::Cons;
    use List::Nil;

    #[test]
    fn test() {
        let a = Rc::new(Cons(42, Rc::new(Nil)));
        let b = Cons(11, Rc::clone(&a));
        let c = Cons(12, Rc::clone(&a));
    }
}

fn rc_panic_example() {
    let mut r = Rc::new(String::new());
    let _r2 = r.clone();
    // fails because we have another reference to the rc, so get_mut will return a None.
    Rc::get_mut(&mut r).unwrap().push_str("foo");
    println!("{}", r);
}

fn box_example() {
    let x = String::from("foo");
    let xb1 = Box::new(x);
    // fails because we have moved x into xb1
    //let xb2 = Box::new(x);
}
