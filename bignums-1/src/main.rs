fn main() {
    let n = Num::from(42);
    println!("{n:?}");
}

#[derive(Debug, PartialEq, Eq)]
struct Num(Vec<u8>);

impl Num {
    fn new() -> Self {
        Self(vec![])
    }
    #[allow(dead_code)]
    fn from(mut v: u64) -> Self {
        let mut res = Self::new();
        let divisor: u64 = 10;
        loop {
            let part = v % divisor;
            res.0.insert(0, part as u8);
            v = v / divisor;
            if v == 0 {
                break;
            }
        }
        res
    }
    #[allow(dead_code)]
    fn add(&self, other: &Num) -> Self {
        // set up the two iterators. the v1 iterator will
        // have at least as many digits as the v2 iterator.
        let mut v1_iter = self.0.iter().rev();
        let mut v2_iter = other.0.iter().rev();
        if v2_iter.len() > v1_iter.len() {
            let tmp = v1_iter;
            v1_iter = v2_iter;
            v2_iter = tmp;
        }
        let mut res = Self::new();
        let mut carry = 0_u8;
        v1_iter.for_each(|num| {
            let other = match v2_iter.next() {
                Some(other) => other,
                None => &0,
            };
            let mut sum = *other + *num + carry;
            carry = sum / 10;
            sum %= 10;
            res.0.insert(0, sum)
        });
        if carry > 0 {
            res.0.insert(0, carry)
        }
        res
    }
}

#[test]
fn test_num_add() {
    assert_eq!(Num::from(75), Num::from(64).add(&Num::from(11)));
    assert_eq!(Num::from(999), Num::from(0).add(&Num::from(999)));
    assert_eq!(Num::from(1000), Num::from(1).add(&Num::from(999)));
    assert_eq!(Num::from(1500), Num::from(750).add(&Num::from(750)));
}

#[test]
fn test_num_from() {
    assert_eq!(Num::from(0), Num(vec![0]));
    assert_eq!(Num::from(643), Num(vec![6, 4, 3]));
    assert_eq!(
        Num::from(1234567890),
        Num(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    );
}
