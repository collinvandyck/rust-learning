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
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        // first fill in the screen dimensions with walls
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let glyph = to_cp437('#');
                ctx.set(x, y, WHITE, BLACK, glyph);
            }
        }
        // then render what is visible in the camera
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if !Self::in_bounds(Point::new(x, y)) {
                    continue;
                }
                let idx = map_idx(x, y);
                let tile = self.tiles[idx];
                let point = camera.translate(Point::new(x, y));
                match tile {
                    TileType::Floor => {
                        let glyph = to_cp437('.');
                        ctx.set(point.x, point.y, WHITE, BLACK, glyph);
                    }
                    TileType::Wall => {
                        let glyph = to_cp437('#');
                        ctx.set(point.x, point.y, WHITE, BLACK, glyph);
                    }
                }
            }
        }
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
