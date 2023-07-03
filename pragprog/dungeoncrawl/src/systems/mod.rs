use crate::prelude::*;

mod player_input;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}
