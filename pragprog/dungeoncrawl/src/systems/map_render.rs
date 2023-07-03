use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    // target the first layer/console.
    draw_batch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            if Map::in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                let offset = Point::new(camera.left_x, camera.top_y);
                let color = ColorPair::new(WHITE, BLACK);
                draw_batch.set(pt - offset, color, glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
