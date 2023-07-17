use num_bigint::BigUint;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Test {
    divisible_by: u8,
    if_true: usize,
    if_false: usize,
    zero: BigUint,
}

impl Test {
    pub fn new(divisible_by: u8, if_true: usize, if_false: usize) -> Self {
        Self {
            divisible_by,
            if_true,
            if_false,
            zero: BigUint::from(0_u64),
        }
    }
    pub fn evaluate(&self, item: &Item) -> usize {
        if item
            .worry
            .divisible_by(self.divisible_by as u64, &self.zero)
        {
            self.if_true
        } else {
            self.if_false
        }
    }
}
