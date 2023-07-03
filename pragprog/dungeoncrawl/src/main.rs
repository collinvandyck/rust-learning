mod map;

use bracket_lib::prelude::*;

struct State {}

impl State {
    fn new() -> Self {
        Self {}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;
    main_loop(ctx, State::new())
}
