use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Item {
    pub worry: BigNum,
}

impl Item {
    pub fn new(worry: u64) -> Self {
        Self {
            worry: BigNum::from(worry),
        }
    }
    pub fn inspect(&mut self, op: &Op, worry_divisor: u64) {
        // first, mutate the worry value on inspection
        op.calculate(&mut self.worry);

        if worry_divisor != 1 {
            self.worry.divide_mut(worry_divisor);
        }
    }
}

#[test]
fn test_divide() {
    assert_eq!(0 / 3, 0);
    assert_eq!(1 / 3, 0);
    assert_eq!(3 / 3, 1);
    assert_eq!(4 / 3, 1);
}
