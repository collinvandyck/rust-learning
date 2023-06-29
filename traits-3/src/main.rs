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
}

#[derive(Debug)]
struct Any<T> {
    val: T,
}

impl<T> ops::Neg for Any<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Any<T>;
    fn neg(self) -> Self::Output {
        Any { val: -self.val }
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
