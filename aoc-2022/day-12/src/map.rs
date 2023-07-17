pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_iter(mut iter: impl Iterator<Item = String>) -> Self {
        let mut tiles = vec![];
        while let Some(line) = iter.next() {
            let row: Vec<Tile> = line.chars().map(|c| Tile(c)).collect();
            tiles.push(row);
        }
        Self { tiles }
    }
    pub fn solve(&self) {}
}

pub struct Tile(char);
