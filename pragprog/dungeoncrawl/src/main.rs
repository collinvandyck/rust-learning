mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        let map = Map::new();
        Self { map }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;
    main_loop(ctx, State::new())
}
