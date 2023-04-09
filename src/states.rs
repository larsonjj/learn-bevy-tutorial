// This example game uses States to separate logic
use bevy::prelude::*;

// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Game,
    // Here the menu is drawn and waiting for player interaction
    MainMenu,
    // During this state, the game over screen is displayed
    GameOver,
}
