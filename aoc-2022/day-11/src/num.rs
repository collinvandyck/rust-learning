use core::fmt::Display;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Num(VecDeque<u8>);

impl Num {
    fn new() -> Self {
        Self(VecDeque::new())
    }
    #[allow(dead_code)]
    pub fn from(mut v: u64) -> Self {
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
    pub fn add(&self, other: &Num) -> Self {
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
        res.trim();
        res
    }
    #[allow(dead_code)]
    pub fn multiply(&self, other: &Num) -> Self {
        let (mut i1, mut i2) = (self, other);
        if i2.0.len() > i1.0.len() {
            (i1, i2) = (i2, i1);
        }
        let mut acc: Vec<Num> = vec![];
        for i2_idx in (0..i2.0.len()).rev() {
            let mut carry = 0_u8;
            let mut tmp = Self::new();
            for i1_idx in (0..i1.0.len()).rev() {
                let i1_dig = i1.0.get(i1_idx).unwrap();
                let i2_dig = i2.0.get(i2_idx).unwrap();
                let product = i1_dig * i2_dig + carry;
                let remainder = product % 10;
                tmp.0.insert(0, remainder);
                carry = product / 10;
            }
            if carry > 0 {
                tmp.0.insert(0, carry);
            }
            acc.push(tmp);
        }
        acc.iter_mut().enumerate().for_each(|(idx, res)| {
            for _ in 0..idx {
                res.0.push_back(0);
            }
        });
        let mut res = acc.into_iter().reduce(|a, b| a.add(&b)).unwrap();
        res.trim();
        res
    }
    fn trim(&mut self) {
        let mut non_zero_seen = false;
        self.0.retain(|x| {
            if non_zero_seen {
                return true;
            }
            if *x != 0 {
                non_zero_seen = true;
            }
            non_zero_seen
        })
    }
    pub fn divide(&self, divisor: u64) -> (Self, u64) {
        let digits = &self.0;
        let mut scratch: VecDeque<u8> = VecDeque::new();
        let mut res = vec![];
        let mut rem = 0_u64;
        for i in 0..digits.len() {
            let digit = digits.get(i).unwrap();
            let digit = *digit;

            // append the digit onto scratch
            scratch.push_back(digit);

            // get scratch as a u64
            let scratch_v: u64 = scratch
                .iter()
                .rev()
                .map(|x| (*x) as u64)
                .enumerate()
                .map(|(idx, v)| v * (10_u64).pow(idx as u32))
                .sum();

            //println!("Scratch: {scratch:?}, asu64:{scratch_v}");

            let div = scratch_v / divisor;
            let mul = divisor * div;
            rem = scratch_v - mul;
            res.push(div as u8);

            // we need to push each digit of the remainder into scratch
            let st = Self::from(rem);
            scratch = st.0;
            //println!("Digit:{digit} divisor:{divisor} mul:{mul} rem:{rem} res:{res:?}");
        }
        let vd = VecDeque::from(res);
        (Self(vd), rem)
    }
    pub fn divisible_by(&self, val: u64) -> bool {
        let (_, remainder) = self.divide(val);
        remainder == 0
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

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.iter().map(|f| format!("{f}")).collect::<String>();
        write!(f, "{s}")
    }
}

impl<'a> From<&'a Num> for String {
    fn from(value: &Num) -> Self {
        value.0.iter().map(|f| format!("{f}")).collect::<String>()
    }
}

#[test]
fn test_num_string() {
    let s: String = (&Num::from(832)).into();
    assert_eq!(s, "832".to_string());
}

#[test]
fn test_long_divide() {
    let num = Num::from(956);
    let divisor = 4;
    let (result, rem) = num.divide(divisor);
    assert_eq!(result, Num::from(239));
    assert_eq!(rem, 0);

    let num = Num::from(957);
    let divisor = 4;
    let (result, rem) = num.divide(divisor);
    assert_eq!(result, Num::from(239));
    assert_eq!(rem, 1);

    let num = Num::from(957);
    let divisor = 1;
    let (result, rem) = num.divide(divisor);
    assert_eq!(result, Num::from(957));
    assert_eq!(rem, 0);
}

#[test]
fn test_divisible_by() {
    assert!(Num::from(86636).divisible_by(1));
    assert!(Num::from(86636).divisible_by(2));
    assert!(!Num::from(86636).divisible_by(3));
    assert!(Num::from(86636).divisible_by(4));
    assert!(!Num::from(86636).divisible_by(5));
}

#[test]
fn test_num_mul() {
    assert_eq!(Num::from(86636), Num::from(3938).multiply(&Num::from(22)));
    assert_eq!(Num::from(7876), Num::from(3938).multiply(&Num::from(2)));
    assert_eq!(Num::from(0), Num::from(1).multiply(&Num::from(0)));
    assert_eq!(Num::from(0), Num::from(0).multiply(&Num::from(0)));
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
