use crate::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(tick_star_spawn_timer.in_set(OnUpdate(GameState::Playing)))
            .add_system(spawn_stars_over_time.in_set(OnUpdate(GameState::Playing)));
    }
}
