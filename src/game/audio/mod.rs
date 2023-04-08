use crate::states::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;

pub struct GameAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_background_audio.in_schedule(OnEnter(GameState::Game)));
    }
}
