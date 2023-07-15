#[derive(Debug)]
pub struct Test {
    divisible_by: i32,
    if_true: i32,
    if_false: i32,
}

impl Test {
    pub fn new(divisible_by: i32, if_true: i32, if_false: i32) -> Self {
        Self {
            divisible_by,
            if_true,
            if_false,
        }
    }
}
