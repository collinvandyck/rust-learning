use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        let glyph = to_cp437('@');
        let point = camera.translate(self.position);
        ctx.set(point.x, point.y, WHITE, BLACK, glyph);
    }
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left | VirtualKeyCode::H => Point::new(-1, 0),
                VirtualKeyCode::Right | VirtualKeyCode::L => Point::new(1, 0),
                VirtualKeyCode::Up | VirtualKeyCode::K => Point::new(0, -1),
                VirtualKeyCode::Down | VirtualKeyCode::J => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(self.position);
            }
        }
    }
}
