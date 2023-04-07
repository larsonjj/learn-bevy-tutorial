use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub exit_game: bool,
}
