use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (x + (y * SCREEN_WIDTH)) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                let tile = self.tiles[idx];
                match tile {
                    TileType::Floor => {
                        let glyph = to_cp437('.');
                        ctx.set(x, y, YELLOW, BLACK, glyph);
                    }
                    TileType::Wall => {
                        let glyph = to_cp437('#');
                        ctx.set(x, y, GREEN, BLACK, glyph);
                    }
                }
            }
        }
    }
}
