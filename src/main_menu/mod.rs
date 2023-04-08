use crate::states::GameState;
use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::MainMenu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)));
    }
}
