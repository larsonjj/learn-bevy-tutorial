mod audio;
mod enemy;
mod events;
mod physics;
mod player;
mod resources;
mod star;
mod states;
mod systems;
mod ui;
pub mod walls;

use crate::states::AppState;
use audio::GameAudioPlugin;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use events::*;
use physics::GamePhysicsPlugin;
use player::PlayerPlugin;
use resources::*;
use star::StarPlugin;
use states::*;
use systems::*;
use ui::GameUIPlugin;
use walls::WallsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>()
            .add_state::<SimulationState>()
            .init_resource::<Score>()
            .add_system(start_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(GameAudioPlugin)
            .add_plugin(GamePhysicsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(WallsPlugin)
            .add_plugin(GameUIPlugin)
            .add_system(update_score.in_set(OnUpdate(AppState::Game)))
            .add_system(handle_game_over_event.in_set(OnUpdate(AppState::Game)))
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(stop_simulation.in_schedule(OnExit(AppState::Game)));
    }
}
