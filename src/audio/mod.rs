use crate::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_background_audio.in_schedule(OnEnter(GameState::Playing)));
    }
}
