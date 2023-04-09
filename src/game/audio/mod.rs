use crate::states::AppState;
use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct GameAudioPlugin;

// This plugin is responsible to control the game audiocargo
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundMusicAudio>()
            .add_system(start_background_audio.in_schedule(OnEnter(AppState::Game)))
            .add_system(stop_background_audio.in_schedule(OnExit(AppState::Game)));
    }
}
