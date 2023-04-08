use crate::states::GameState;
use bevy::prelude::*;

pub mod components;
mod events;
mod systems;

use events::*;
use systems::*;

const ENEMY_SPEED: f32 = 150.;
const ENEMY_SIZE: f32 = 64.;
const NUMBER_OF_ENEMIES: usize = 3;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHitWallEvent>()
            .add_system(spawn_enemies.in_schedule(OnEnter(GameState::Game)))
            .add_system(move_enemy_controller.in_set(OnUpdate(GameState::Game)))
            .add_system(play_enemy_wall_hit_sound.in_set(OnUpdate(GameState::Game)))
            .add_system(check_for_world_collisions.in_set(OnUpdate(GameState::Game)));
    }
}
