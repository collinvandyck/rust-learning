use crate::prelude::*;

#[derive(Debug)]
pub struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    pub fn new(divisible_by: u64, if_true: usize, if_false: usize) -> Self {
        Self {
            divisible_by,
            if_true,
            if_false,
        }
    }
    pub fn evaluate(&self, item: &Item) -> usize {
        if item.worry % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
