#[derive(Debug)]
pub struct Registers {
    pub x: i64,
}

impl Registers {
    pub fn new() -> Self {
        Self { x: 1 }
    }
}
