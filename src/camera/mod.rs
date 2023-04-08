use bevy::prelude::*;

mod systems;

use systems::*;

pub struct CameraPlugin;

// This plugin is responsible to control the game audio
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_camera.on_startup());
    }
}
