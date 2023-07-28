#![allow(unused_variables, dead_code, unused_imports)]
use std::rc::Rc;

fn main() {}

mod rc {
    use std::rc::Rc;
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::*;

    #[test]
    fn test() {
        let a: Rc<List> = Rc::new(Cons(42, Rc::new(Nil)));
        let b: List = Cons(11, Rc::clone(&a));
        let c: List = Cons(12, Rc::clone(&a));
    }
    #[test]
    fn test_get_mut() {
        let mut r = Rc::new(String::from("foobar"));
        assert_eq!(Rc::get_mut(&mut r), Some(&mut "foobar".to_string()));
        let holdup = r.clone();
        assert_eq!(Rc::get_mut(&mut r), None);
        drop(holdup);
        assert_eq!(Rc::get_mut(&mut r), Some(&mut "foobar".to_string()));
    }
    #[test]
    fn test_counts() {
        let a: Rc<List> = Rc::new(Cons(42, Rc::new(Nil)));
        assert_eq!(Rc::strong_count(&a), 1);
        let b: List = Cons(11, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 2);
        let c: List = Cons(12, Rc::clone(&a));
        assert_eq!(Rc::strong_count(&a), 3);
    }
}

fn box_example() {
    let x = String::from("foo");
    let xb1 = Box::new(x);
    // fails because we have moved x into xb1
    //let xb2 = Box::new(x);
}
