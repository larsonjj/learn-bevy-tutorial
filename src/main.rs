// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
mod camera;
mod game;
pub mod loading;
mod main_menu;
pub mod states;
mod systems;

use actions::ActionsPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use camera::CameraPlugin;
use game::GamePlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::*;

fn main() {
    let mut binding = App::new();

    let app = binding
        // Bevy
        .add_state::<AppState>()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Learn Bevy Tutorial".to_string(),
                resolution: (960., 540.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        // App Systems
        .add_system(set_window_icon.on_startup())
        .add_system(handle_app_exit_event)
        // My Plugins
        .add_plugin(LoadingPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin);

    #[cfg(debug_assertions)]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    // Start the app
    app.run();
}
