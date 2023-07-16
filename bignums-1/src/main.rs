use std::collections::VecDeque;

fn main() {
    let n = Num::from(42);
    println!("{n:?}");
}

#[derive(Debug, PartialEq, Eq)]
struct Num(VecDeque<u8>);

impl Num {
    fn new() -> Self {
        Self(VecDeque::new())
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
        let (i1, mut i2) = self.iters(other);
        let mut res = Self::new();
        let mut carry = 0_u8;
        i1.for_each(|num| {
            let other = match i2.next() {
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

    // how to multiply
    //   3938
    // x   22
    // ------
    //
    // 2 * 8 = 16     => 6 carry 1
    // 2 * 3 = 6 + c1 => 7
    // 2 * 9 = 18     => 8 carry 1
    // 2 * 3 = 6 + c1 => 7
    // => [7,8,7,6]
    //
    // Now do it for the next number
    //
    // 2 * 8 = 16     => 6 carry 1
    // 2 * 3 = 6 + c1 => 7
    // 2 * 9 = 18     => 8 carry 1
    // 2 * 3 = 6 + c1 => 7
    // => [7,8,7,6]
    // but multiply it by 10 (add a zero)
    // => [7,8,7,6,0]
    //
    // Then we will add these numbers together
    // 7876 + 78760 => 86,636
    //
    #[allow(dead_code)]
    fn multiply(&self, other: &Num) -> Self {
        let (i1, i2) = self.iters(other); // i1 is at least as big as i2
        let (i1, i2) = (i2, i1); // i2 is at least as big as i1
        let mut carry = 0_u8;
        let mut res = Self::new();
        res
    }

    // returns two reversed iterators. The first iterator is guaranteed
    // to be at least as long as the second iterator.
    fn iters<'a>(
        &'a self,
        other: &'a Num,
    ) -> (impl Iterator<Item = &u8>, impl Iterator<Item = &u8>) {
        let v1 = self.0.iter().rev();
        let v2 = other.0.iter().rev();
        if v2.len() > v1.len() {
            (v2, v1)
        } else {
            (v1, v2)
        }
    }
}

#[test]
fn test_num_add() {
    assert_eq!(Num::from(75), Num::from(64).add(&Num::from(11)));
    assert_eq!(Num::from(999), Num::from(0).add(&Num::from(999)));
    assert_eq!(Num::from(1000), Num::from(1).add(&Num::from(999)));
    assert_eq!(Num::from(1000), Num::from(999).add(&Num::from(1)));
    assert_eq!(Num::from(1500), Num::from(750).add(&Num::from(750)));
}

#[test]
fn test_num_from() {
    assert_eq!(Num::from(0), Num(VecDeque::from([0])));
    assert_eq!(Num::from(643), Num(VecDeque::from([6, 4, 3])));
    assert_eq!(
        Num::from(1234567890),
        Num(VecDeque::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]))
    );
}
