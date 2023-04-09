use super::states::AppState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod resources;

use resources::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading);
    }
}
