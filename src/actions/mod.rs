use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::states::AppState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(handle_escape)
            .add_system(handle_movement_actions.in_set(OnUpdate(AppState::Game)))
            .add_system(handle_pause.in_set(OnUpdate(AppState::Game)));
    }
}
