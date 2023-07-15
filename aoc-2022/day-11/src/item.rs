#[derive(Debug)]
pub struct Item {
    score: i32,
}

impl Item {
    pub fn new(score: i32) -> Self {
        Self { score }
    }
}
