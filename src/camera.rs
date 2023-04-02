use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

// This plugin is responsible to control the game audio
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_camera.on_startup());
    }
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1000.0),
        ..default()
    });
}
