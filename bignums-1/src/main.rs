use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
struct Num(Vec<u64>);

impl Num {
    fn from(mut v: u64) -> Self {
        let mut vd = Vec::new();
        let divisor: u64 = 10;
        loop {
            println!("v: {v} divisor: {divisor}");
            let part = v % divisor;
            vd.insert(0, part);
            v = v / divisor;
            if v == 0 {
                break;
            }
        }
        Self(vd)
    }
}

#[test]
fn test_num() {
    assert_eq!(Num::from(0), Num(vec![0]));
    assert_eq!(Num::from(643), Num(vec![6, 4, 3]));
    assert_eq!(
        Num::from(1234567890),
        Num(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
}
