use bevy::prelude::*;

#[derive(Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub exit_game: bool,
    pub pause: bool,
}

impl Default for Actions {
    fn default() -> Actions {
        Actions {
            player_movement: None,
            exit_game: false,
            pause: false,
        }
    }
}
