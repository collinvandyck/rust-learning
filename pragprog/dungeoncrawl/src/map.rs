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

#[allow(clippy::cast_sign_loss)]
pub fn map_idx(x: i32, y: i32) -> usize {
    (x + (y * SCREEN_WIDTH)) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    pub fn in_bounds(point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        Self::in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if Self::in_bounds(point) {
            Some(map_idx(point.x, point.y))
        } else {
            None
        }
    }
}
