const MULTIPLE: u64 = 100;

#[derive(Copy, Clone, Debug)]
pub struct Value {
    multiple: u64,
    remainder: u64,
}

impl Value {
    pub fn from_u64(v: u64) -> Self {
        let multiple = v / MULTIPLE;
        let remainder = v % MULTIPLE;
        Self::new(multiple, remainder)
    }
    fn new(multiple: u64, remainder: u64) -> Self {
        Self {
            multiple,
            remainder,
        }
    }
    fn add(&self, other: &Value) -> Self {
        let mut multiple = self.multiple + other.multiple;
        let mut remainder = self.remainder + other.remainder;
        multiple += remainder / MULTIPLE;
        remainder = remainder % MULTIPLE;
        Self {
            multiple,
            remainder,
        }
    }
    // multiple: 100
    // first : 250 (multiple: 2, remainder: 50)
    // second: 575 (multiple: 5, remainder: 75)
    //
    // expected: 143,750 (multple: 1437, remainder: 50)
    // remainders multiplied: 3,750
    // additional multiple: 37
    //
    // i need a multiple of 1400
    // (5+2)*2 = 14 * 100
    //
    //
    fn mul(&self, other: &Value) -> Self {
        let mul_remain = self.remainder * other.remainder;
        let remainder = mul_remain % MULTIPLE;
        let additional_multiple = mul_remain / MULTIPLE;
        let multiple = (self.multiple + other.multiple) * 2 * MULTIPLE;
        let multiple = multiple + additional_multiple;
        Self {
            multiple,
            remainder,
        }
    }
}

#[test]
fn test_add() {}
