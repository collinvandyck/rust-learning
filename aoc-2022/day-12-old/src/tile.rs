#[derive(Clone, Copy, Debug)]
pub struct Tile(pub char);

impl Tile {
    pub fn can_move_to(&self, other: &Tile) -> bool {
        let self_n = self.0 as i64;
        let other_n = other.0 as i64;
        // other_n can be at most 1 more than self_n
        self_n - other_n >= -1
    }
}
