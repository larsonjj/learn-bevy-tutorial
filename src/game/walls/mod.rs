use crate::states::GameState;
use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub const WALLS_MARGIN: f32 = 5.0;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_walls.in_schedule(OnEnter(GameState::Game)));
    }
}
