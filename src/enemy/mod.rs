use crate::GameState;
use bevy::prelude::*;

pub mod components;
mod events;
mod systems;

use events::*;
use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHitWallEvent>()
            .add_system(spawn_enemies.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_enemy_controller.in_set(OnUpdate(GameState::Playing)))
            .add_system(play_enemy_wall_hit_sound.in_set(OnUpdate(GameState::Playing)))
            .add_system(check_for_world_collisions.in_set(OnUpdate(GameState::Playing)));
    }
}
