mod actions;
mod asset_loader;
mod audio;
mod camera;
mod enemy;
mod events;
mod main_menu;
mod physics;
mod player;
mod resources;
mod star;
mod states;
mod systems;
mod walls;

use crate::actions::ActionsPlugin;
use crate::asset_loader::AssetLoaderPlugin;
use crate::audio::InternalAudioPlugin;
use crate::camera::CameraPlugin;
use crate::enemy::EnemyPlugin;
use crate::events::*;
use crate::main_menu::MainMenuPlugin;
use crate::physics::InternalPhysicsPlugin;
use crate::player::PlayerPlugin;
use crate::resources::*;
use crate::star::StarPlugin;
use crate::states::GameState;
use crate::systems::*;
use crate::walls::WallsPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameOverEvent>()
            .init_resource::<Score>()
            .add_plugin(CameraPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(InternalPhysicsPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(WallsPlugin)
            .add_system(control_game_exit_event)
            .add_system(update_score.in_set(OnUpdate(GameState::Playing)))
            .add_system(handle_game_over_event.in_set(OnUpdate(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
