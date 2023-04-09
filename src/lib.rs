// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
mod camera;
mod game;
mod loading;
mod main_menu;
pub mod states;
pub mod systems;

use crate::actions::ActionsPlugin;
use crate::camera::CameraPlugin;
use crate::game::GamePlugin;
use crate::loading::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app
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
    }
}
