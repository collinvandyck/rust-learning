#[derive(Debug)]
pub struct Item {
    worry: i32,
}

impl Item {
    pub fn new(worry: i32) -> Self {
        Self { worry }
    }
}
