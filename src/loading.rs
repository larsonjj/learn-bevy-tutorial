use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/chosic/fluffing-a-duck.ogg")]
    pub background_music: Handle<AudioSource>,
    #[asset(path = "audio/kenney/interface/pluck_001.ogg")]
    pub enemy_direction_changed_1: Handle<AudioSource>,
    #[asset(path = "audio/kenney/interface/pluck_002.ogg")]
    pub enemy_direction_changed_2: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/kenney/rolling_ball/ball_blue_large.png")]
    pub texture_ball: Handle<Image>,
    #[asset(path = "textures/kenney/rolling_ball/ball_red_large.png")]
    pub enemy_ball: Handle<Image>,
    #[asset(path = "textures/kenney/rolling_ball/star.png")]
    pub star: Handle<Image>,
}
