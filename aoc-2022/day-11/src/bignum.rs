use num_bigint::BigUint;
use rug::ops::Pow;
use std::{fmt::Display, ops::Rem};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BigNum(rug::Integer);

impl BigNum {
    pub fn from(v: u64) -> Self {
        Self(rug::Integer::from(v))
    }
    pub fn add_num(&mut self, other: u64) {
        self.0 += other;
    }
    pub fn multiply_by(&mut self, other: u64) {
        self.0 *= other;
    }
    pub fn square(&mut self) {
        self.0 = self.0.clone().pow(2)
    }
    pub fn divide_mut(&mut self, other: u64) {
        self.0 /= other;
    }
    pub fn divisible_by(&self, val: u64, _zero: &BigUint) -> bool {
        self.0.is_divisible(&rug::Integer::from(val))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OldNum(BigUint);

impl OldNum {
    pub fn from(v: u64) -> Self {
        Self(BigUint::from(v))
    }
    pub fn add_num(&mut self, other: u64) {
        self.0 += other;
    }
    pub fn multiply_by(&mut self, other: u64) {
        self.0 *= other;
    }
    pub fn square(&mut self) {
        self.0 = self.0.pow(2_u32);
    }
    pub fn divide_mut(&mut self, other: u64) {
        self.0 /= other;
    }
    pub fn divisible_by(&self, val: u64, zero: &BigUint) -> bool {
        let r = (&self.0).rem(val);
        &r == zero
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}
