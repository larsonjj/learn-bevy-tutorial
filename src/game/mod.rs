mod audio;
mod enemy;
mod events;
mod physics;
mod player;
mod resources;
mod star;
mod systems;
pub mod walls;

use crate::states::GameState;
use audio::GameAudioPlugin;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use events::*;
use physics::GamePhysicsPlugin;
use player::PlayerPlugin;
use resources::*;
use star::StarPlugin;
use systems::*;
use walls::WallsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .init_resource::<Score>()
            .add_plugin(GameAudioPlugin)
            .add_plugin(GamePhysicsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(WallsPlugin)
            .add_system(update_score.in_set(OnUpdate(GameState::Game)))
            .add_system(handle_game_over_event.in_set(OnUpdate(GameState::Game)));
    }
}
