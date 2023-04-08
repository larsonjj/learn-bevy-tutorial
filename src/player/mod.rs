use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod events;
mod systems;

use events::*;
use systems::*;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEnemyEvent>()
            .add_event::<PlayerStarPickupEvent>()
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player_controller.in_set(OnUpdate(GameState::Playing)))
            .add_system(play_player_hit_enemy_sound.in_set(OnUpdate(GameState::Playing)))
            .add_system(play_star_pickup_sound.in_set(OnUpdate(GameState::Playing)))
            .add_system(check_for_world_collisions.in_set(OnUpdate(GameState::Playing)));
    }
}
