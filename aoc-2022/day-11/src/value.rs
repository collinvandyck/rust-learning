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
    fn as_u64(&self) -> u64 {
        self.multiple * MULTIPLE + self.remainder
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
    // one: 150 (multiple: 1 remainder: 50)
    // two: 575 (multiple: 5 remainder: 75)
    // expected: 86,250
    //      multiple: 862
    //      remainder: 50
    // remainder product: 3,750
    //      multiple: 37
    //      remainder: 50
    // expected multiple: 862
    //      - remaining multiple = 825
    // adding the multiples
    //      (5+1)*2*100 =
    //
    // mul1: 1
    // mul2: 5
    // i need to combine these somehow to produce 825
    //
    // 1 * 100
    // 5 * 100
    //
    // (1 * 100) + (100 * 100)
    // (5 * 100) + (100 * 100)
    //
    // 1+5=6 * 100 = 600 (needs 225)
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
fn test_add() {
    let (one, two) = (150_u64, 575_u64);
    let expected = one + two;
    let one = Value::from_u64(one);
    let two = Value::from_u64(two);
    let add = dbg!(one.add(&two));
    assert_eq!(add.as_u64(), expected);
}

#[test]
fn test_mul() {
    let (one, two) = (150_u64, 575_u64);
    let expected = one * two;
    let one = Value::from_u64(one);
    let two = Value::from_u64(two);
    let add = dbg!(one.mul(&two));
    assert_eq!(add.as_u64(), expected);
}
