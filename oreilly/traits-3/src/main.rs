use std::cmp::{self, Ordering, PartialEq};
use std::ops::{self};

fn main() {
    println!("Hello, world!");
    let mut f = Bool { val: true };
    f = dbg!(f);
    let g = -f;
    dbg!(g);

    let mut any = Any { val: 5 };
    any = dbg!(any);
    let any = -any;
    dbg!(any);

    let any1 = Any { val: Blop {} };
    let any2 = Any { val: Blop {} };
    assert_eq!(any1, any2);
    assert!(any1 <= any2);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Blop {}

#[derive(Debug, PartialEq)]
struct Any<T> {
    val: T,
}

impl<T> cmp::Eq for Any<T> where T: cmp::PartialEq {}

impl<T> ops::Neg for Any<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Any<T>;
    fn neg(self) -> Self::Output {
        Any { val: -self.val }
    }
}

impl<T> cmp::PartialOrd for Any<T>
where
    T: cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.val < other.val {
            Some(Ordering::Less)
        } else if self.val == other.val {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

#[derive(Debug)]
struct Bool {
    val: bool,
}

impl ops::Neg for Bool {
    type Output = Bool;
    fn neg(self) -> Self::Output {
        Self { val: !self.val }
    }
}
