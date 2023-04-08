use crate::states::AppState;
use bevy::prelude::*;

pub mod components;
mod events;
mod systems;

use events::*;
use systems::*;

use super::states::SimulationState;

const PLAYER_SPEED: f32 = 300.;
const PLAYER_SIZE: f32 = 64.;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `AppState::Game`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerHitEnemyEvent>()
            .add_event::<PlayerStarPickupEvent>()
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                move_player_controller
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                play_player_hit_enemy_sound
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                play_star_pickup_sound
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                check_for_world_collisions
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
