use crate::prelude::*;

#[derive(Debug)]
pub struct Item {
    pub worry: i32,
}

impl Item {
    pub fn new(worry: i32) -> Self {
        Self { worry }
    }
    pub fn inspect(&mut self, op: &Op) {
        // first, mutate the worry value on inspection
        self.worry = op.calculate(self.worry);

        // divide by three b/c no items are damaged
        self.worry /= 3;
    }
}

#[test]
fn test_divide() {
    assert_eq!(0 / 3, 0);
    assert_eq!(1 / 3, 0);
    assert_eq!(3 / 3, 1);
    assert_eq!(4 / 3, 1);
}
