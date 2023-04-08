use crate::GameState;
use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_walls.in_schedule(OnEnter(GameState::Playing)));
    }
}