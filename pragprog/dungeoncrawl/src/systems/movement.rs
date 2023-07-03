use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    entity: &Entity,
    want_move: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // replace the Point component of the target entity.
        commands.add_component(want_move.entity, want_move.destination);
        // if it is a player, then update the camera after the move.
        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    // remove the message once it is processed to avoid re-processing it.
    commands.remove(*entity);
}
