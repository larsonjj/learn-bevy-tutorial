// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod actions;
pub mod asset_loader;
mod camera;
mod game;
mod main_menu;
pub mod states;
mod systems;

use actions::ActionsPlugin;
use asset_loader::AssetLoaderPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use camera::CameraPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::GameState;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let mut binding = App::new();

    let app = binding
        .add_state::<GameState>()
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
        .add_plugin(AssetLoaderPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_system(set_window_icon.on_startup());

    #[cfg(debug_assertions)]
    {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    // Start the app
    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
