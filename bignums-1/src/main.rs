fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
struct Num(Vec<u64>);

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
            res.0.insert(0, part);
            v = v / divisor;
            if v == 0 {
                break;
            }
        }
        res
    }
    // 56
    // 78
    //
    // 14 -> 10 + 4
    fn add(&self, other: &Num) -> Self {
        let v1 = &self.0;
        let v2 = &other.0;
        let iter = std::iter::zip(v1.iter().rev(), v2.iter().rev());
        let mut res = Self::new();
        let mut carry = 0_u64;
        iter.for_each(|(n1, n2)| {
            let mut sum = n1 + n2 + carry;
            if sum >= 10 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }
            res.0.insert(0, sum)
        });
        res
    }
}

#[test]
fn test_num_add() {
    let n1 = Num::from(64);
    let n2 = Num::from(11);
    assert_eq!(Num::from(75), n1.add(&n2));
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
