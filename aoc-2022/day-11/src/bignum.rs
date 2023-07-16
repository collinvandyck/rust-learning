use std::fmt::Display;

use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BigNum(BigUint);

impl BigNum {
    pub fn from(v: u64) -> Self {
        BigNum(BigUint::from(v))
    }
    pub fn add_num(&mut self, other: u64) {
        self.0 += other;
    }
    pub fn multiply_by(&mut self, other: u64) {
        self.0 *= other;
    }
    pub fn multiply(&mut self, other: &BigNum) {
        self.0 *= &other.0;
    }
    pub fn divide_mut(&mut self, other: u64) {
        self.0 /= other;
    }
    pub fn divide(&self, other: u64) -> (Self, u64) {
        let remainder = self.0.clone() % other;
        let remainder = remainder.to_u64_digits();
        let remainder = if remainder.len() == 0 {
            0
        } else {
            remainder[0]
        };
        let div = BigNum(self.0.clone() / other);
        (div, remainder)
    }
    pub fn divisible_by(&self, val: u64) -> bool {
        self.0.clone() % val == BigUint::from(0u64)
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}
