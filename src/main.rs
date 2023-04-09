// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use learn_bevy_tutorial::states::*;
use learn_bevy_tutorial::systems::*;
use learn_bevy_tutorial::LibPlugin;

fn main() {
    App::new()
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
        .add_plugin(LibPlugin)
        .run();
}
